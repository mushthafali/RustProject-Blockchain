use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use serde::{Deserialize, Serialize}; // <-- Tambahkan Serialize di sini
use reqwest::Client;
use chrono::TimeZone;
use std::fs::OpenOptions;
use std::io::Write;
use std::env;
use dotenv::dotenv;
use eyre::Result; 

// --- Imports untuk ethers-rs ---
use ethers::{
    providers::{Provider, Http}, 
    signers::{LocalWallet, Signer}, 
    core::types::Address, 
    middleware::SignerMiddleware, 
};
use std::sync::Arc; 
use tokio::time::{Duration, timeout}; 

// Import ABI yang digenerate otomatis oleh build.rs
mod abi;
use abi::SensorDataRecorder; 

// <-- Tambahkan import untuk broadcast channel dan error-nya
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError; // Untuk mengatasi error E0603


// Struktur data SensorData, sama seperti yang diterima dari sensor_client
#[derive(Deserialize, Debug, Clone, Serialize)] // <-- Tambahkan Serialize di sini!
struct SensorData {
    timestamp: String,
    sensor_id: String,
    location: String,
    process_stage: String,
    temperature_celsius: f32,
    humidity_percent: f32,
}

// Fungsi helper untuk meng-escape tag dalam InfluxDB Line Protocol
fn escape_tag(value: &str) -> String {
    value
        .replace('\\', "\\\\") 
        .replace(' ', "\\ ")
        .replace(',', "\\,")
        .replace('=', "\\=")
}

// Fungsi asinkron untuk mengirim data sensor ke EVM (Ganache)
async fn send_data_to_evm(
    contract: &SensorDataRecorder<SignerMiddleware<Arc<Provider<Http>>, LocalWallet>>,
    sensor_data: &SensorData, 
    log_file: &mut std::fs::File, 
) -> Result<()> {
    let timestamp_log = chrono::FixedOffset::east_opt(7 * 3600) 
        .unwrap()
        .from_utc_datetime(&chrono::Utc::now().naive_utc()).to_rfc3339();

    let parsed_ts_unix_nano = match chrono::DateTime::parse_from_rfc3339(&sensor_data.timestamp) {
        Ok(dt) => dt.timestamp_nanos_opt().unwrap_or(0) as u64,
        Err(_) => {
            println!("[{}] ⚠️ Timestamp sensor tidak valid, menggunakan waktu UTC sekarang", timestamp_log);
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64
        }
    };

    let temp_scaled = (sensor_data.temperature_celsius * 100.0) as i32;
    let humidity_scaled = (sensor_data.humidity_percent * 100.0) as u32;

    println!("[{}] Memanggil recordSensorData dengan: temp_scaled={}, hum_scaled={}",
             timestamp_log, temp_scaled, humidity_scaled);

    let call = contract.record_sensor_data(
        parsed_ts_unix_nano,
        sensor_data.sensor_id.clone(),
        sensor_data.location.clone(),
        sensor_data.process_stage.clone(),
        temp_scaled,
        humidity_scaled,
    );

    let send_result_with_timeout = timeout(Duration::from_secs(30), call.send()).await;

    let final_result = match send_result_with_timeout {
        Ok(Ok(tx)) => { 
            println!("[{}] ✅ Transaksi dikirim ke EVM: {:?}", timestamp_log, tx.tx_hash());
            writeln!(log_file, "[{}] Transaksi dikirim ke EVM: {:?}", timestamp_log, tx.tx_hash())?;

            let confirm_result_with_timeout = timeout(Duration::from_secs(60), async {
                tx.await
            }).await;

            match confirm_result_with_timeout {
                Ok(Ok(receipt_opt)) => { 
                    if let Some(receipt) = receipt_opt {
                        println!("[{}] ✅ Transaksi EVM dikonfirmasi! Block: {:?}, Status: {:?}",
                                 timestamp_log, receipt.block_number, receipt.status);
                        writeln!(log_file, "[{}] Transaksi EVM dikonfirmasi! Block: {:?}, Status: {:?}",
                                 timestamp_log, receipt.block_number, receipt.status)?;
                    } else {
                        println!("[{}] ⚠️ Transaksi EVM belum dikonfirmasi atau hilang (timeout konfirmasi?).", timestamp_log);
                        writeln!(log_file, "[{}] Transaksi EVM belum dikonfirmasi atau hilang (timeout konfirmasi?).", timestamp_log)?;
                    }
                    Ok(())
                },
                Ok(Err(e)) => { 
                    println!("[{}] ❌ Gagal konfirmasi transaksi EVM: {}", timestamp_log, e);
                    writeln!(log_file, "[{}] Gagal konfirmasi transaksi EVM: {}", timestamp_log, e)?;
                    Err(e.into())
                },
                Err(_) => { 
                    println!("[{}] ❌ Timeout saat menunggu konfirmasi transaksi EVM.", timestamp_log);
                    writeln!(log_file, "[{}] Timeout saat menunggu konfirmasi transaksi EVM.", timestamp_log)?;
                    Err(eyre::eyre!("Timeout during EVM transaction confirmation"))
                }
            }
        },
        Ok(Err(e)) => { 
            println!("[{}] ❌ Gagal kirim transaksi ke EVM: {}", timestamp_log, e);
            writeln!(log_file, "[{}] Gagal kirim transaksi ke EVM: {}", timestamp_log, e)?;
            Err(e.into())
        },
        Err(_) => { 
            println!("[{}] ❌ Timeout saat mencoba mengirim transaksi ke EVM.", timestamp_log);
            writeln!(log_file, "[{}] Timeout saat mencoba mengirim transaksi ke EVM.", timestamp_log)?;
            Err(eyre::eyre!("Timeout during EVM transaction sending"))
        }
    };
    final_result 
}

// Fungsi baru untuk menangani setiap koneksi klien
// Ini akan membedakan antara klien sensor dan klien GUI
async fn handle_connection(
    socket: TcpStream,
    addr: std::net::SocketAddr,
    http_client: Client,
    influx_url: String,
    influx_token: String,
    contract: SensorDataRecorder<SignerMiddleware<Arc<Provider<Http>>, LocalWallet>>,
    mut log_file: std::fs::File, 
    tx_sensor_data: broadcast::Sender<String>, // Pengirim broadcast channel
) -> Result<()> {
    let wib = chrono::FixedOffset::east_opt(7 * 3600).unwrap();
    let timestamp_conn = wib.from_utc_datetime(&chrono::Utc::now().naive_utc()).to_rfc3339();
    
    let (reader, mut writer) = socket.into_split();
    let mut lines = BufReader::new(reader).lines();

    let mut is_gui_client = false;

    // Baca baris pertama untuk identifikasi klien (dengan timeout)
    let identification_line_result = timeout(Duration::from_secs(5), lines.next_line()).await;
    
    match identification_line_result {
        Ok(Ok(Some(line_str))) => {
            let id_line = line_str.trim();
            if id_line == "GUI_CLIENT" { // String identifikasi dari GUI
                is_gui_client = true;
                println!("[{}] Klien {} adalah GUI.", timestamp_conn, addr);
                writeln!(log_file, "[{}] Klien {} adalah GUI.", timestamp_conn, addr)?;
                
                // Jika ini GUI, buat receiver dan mulai mendengarkan broadcast
                let mut rx_sensor_data = tx_sensor_data.subscribe();
                tokio::spawn(async move {
                    loop {
                        match rx_sensor_data.recv().await {
                            Ok(data_json) => {
                                // Kirim data JSON yang diterima dari channel ke GUI
                                if let Err(e) = writer.write_all(data_json.as_bytes()).await {
                                    println!("[{}] ❌ Gagal kirim data ke GUI {}: {}", 
                                             chrono::FixedOffset::east_opt(7 * 3600).unwrap().from_utc_datetime(&chrono::Utc::now().naive_utc()).to_rfc3339(), addr, e);
                                    break; 
                                }
                                if let Err(e) = writer.flush().await {
                                    println!("[{}] ❌ Gagal flush ke GUI {}: {}", 
                                             chrono::FixedOffset::east_opt(7 * 3600).unwrap().from_utc_datetime(&chrono::Utc::now().naive_utc()).to_rfc3339(), addr, e);
                                    break;
                                }
                            },
                            Err(RecvError::Lagged(n)) => { // <-- Perbaikan error E0603 di sini
                                println!("[{}] ⚠️ GUI client {} ketinggalan {} pesan di broadcast channel.",
                                         chrono::FixedOffset::east_opt(7 * 3600).unwrap().from_utc_datetime(&chrono::Utc::now().naive_utc()).to_rfc3339(), addr, n);
                            },
                            Err(RecvError::Closed) => { // <-- Perbaikan error E0603 di sini
                                println!("[{}] ❌ Broadcast channel ditutup untuk GUI {}.",
                                         chrono::FixedOffset::east_opt(7 * 3600).unwrap().from_utc_datetime(&chrono::Utc::now().naive_utc()).to_rfc3339(), addr);
                                break;
                            }
                        }
                    }
                    Ok::<(), tokio::io::Error>(()) // Return tipe untuk spawned task
                });
            } else {
                // Klien sensor atau klien lain, log dan lanjutkan
                println!("[{}] Klien {} adalah Sensor/Lain: {}", timestamp_conn, addr, id_line);
                writeln!(log_file, "[{}] Klien {} adalah Sensor/Lain: {}", timestamp_conn, addr, id_line)?;
            }
        },
        Ok(Ok(None)) => { 
            println!("[{}] 🔌 Klien {} memutuskan koneksi saat identifikasi.", timestamp_conn, addr);
            writeln!(log_file, "[{}] Klien {} memutuskan koneksi saat identifikasi.", timestamp_conn, addr)?;
            return Ok(()); 
        },
        Ok(Err(e)) => { 
            println!("[{}] ❌ Error saat membaca identifikasi dari klien {}: {}", timestamp_conn, addr, e);
            writeln!(log_file, "[{}] Error saat membaca identifikasi dari klien {}: {}", timestamp_conn, addr, e)?;
            return Err(e.into());
        },
        Err(_) => { 
            println!("[{}] ❌ Timeout (5s) saat menunggu identifikasi dari klien {}. Menutup koneksi.", timestamp_conn, addr);
            writeln!(log_file, "[{}] Timeout (5s) saat menunggu identifikasi dari klien {}. Menutup koneksi.", timestamp_conn, addr)?;
            return Ok(());
        }
    }

    // Loop untuk membaca data dari klien (khususnya sensor client)
    // Klien GUI tidak diharapkan mengirim data sensor setelah identifikasi
    if !is_gui_client {
        loop {
            let timestamp_log = chrono::FixedOffset::east_opt(7 * 3600)
                .unwrap()
                .from_utc_datetime(&chrono::Utc::now().naive_utc())
                .to_rfc3339();

            let line_result = timeout(Duration::from_secs(60), lines.next_line()).await;

            let line = match line_result {
                Ok(Ok(Some(line_str))) => line_str, 
                Ok(Ok(None)) => { 
                    println!("[{}] 🔌 Klien {} memutuskan koneksi secara graceful.", timestamp_log, addr);
                    writeln!(log_file, "[{}] Klien {} memutuskan koneksi secara graceful.", timestamp_log, addr)?;
                    break; 
                },
                Ok(Err(e)) => { 
                    println!("[{}] ❌ Error saat membaca dari klien {}: {}", timestamp_log, addr, e);
                    writeln!(log_file, "[{}] Error saat membaca dari klien {}: {}", timestamp_log, addr, e)?;
                    break; 
                },
                Err(_) => { 
                    println!("[{}] ❌ Timeout (60s) saat menunggu data dari klien {}. Menutup koneksi.", timestamp_log, addr);
                    writeln!(log_file, "[{}] Timeout (60s) saat menunggu data dari klien {}. Menutup koneksi.", timestamp_log, addr)?;
                    break; 
                }
            };

            match serde_json::from_str::<SensorData>(&line) {
                Ok(data) => {
                    println!("[{}] 📥 Data diterima dari {}: {:?}", timestamp_log, addr, data);
                    writeln!(log_file, "[{}] Data diterima dari {}: {:?}", timestamp_log, addr, data)?;

                    // Validasi nilai sensor berdasarkan rentang yang masuk akal
                    if data.temperature_celsius < -40.0 || data.temperature_celsius > 125.0 ||
                        data.humidity_percent < 0.0 || data.humidity_percent > 100.0 {
                        println!(
                            "[{}] ⚠️ Data tidak valid dari {}: Suhu {:.1} °C, Kelembapan {:.1} %",
                            timestamp_log, addr, data.temperature_celsius, data.humidity_percent
                        );
                        writeln!(
                            log_file,
                            "[{}] Data tidak valid dari {}: Suhu {:.1} °C, Kelembapan {:.1} %",
                            timestamp_log, addr, data.temperature_celsius, data.humidity_percent
                        )?;
                        continue; 
                    }

                    // <-- Kunci modifikasi: Kirim data JSON ke broadcast channel
                    let data_json_string = serde_json::to_string(&data)?; // Konversi SensorData ke JSON string
                    if let Err(e) = tx_sensor_data.send(format!("{}\n", data_json_string)) {
                        println!("[{}] ❌ Gagal kirim ke broadcast channel: {}", timestamp_log, e);
                        writeln!(log_file, "[{}] Gagal kirim ke broadcast channel: {}", timestamp_log, e)?;
                    }

                    // Parse timestamp sensor untuk format InfluxDB (nanodetik)
                    let parsed_ts_influx = match chrono::DateTime::parse_from_rfc3339(&data.timestamp) {
                        Ok(dt) => dt.timestamp_nanos_opt().unwrap_or_else(|| {
                            println!("[{}] ⚠️ Timestamp sensor invalid (InfluxDB), pakai waktu sekarang", timestamp_log);
                            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
                        }),
                        Err(e) => {
                            println!("[{}] ⚠️ Gagal parse timestamp '{}' (InfluxDB): {}", timestamp_log, data.timestamp, e);
                            writeln!(log_file, "[{}] Gagal parse timestamp '{}' (InfluxDB): {}", timestamp_log, data.timestamp, e)?;
                            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
                        }
                    };

                    // Buat string Line Protocol untuk InfluxDB
                    let influx_line = format!(
                        "SHT20,sensor_id={},location={},process_stage={} temperature_celsius={},humidity_percent={} {}",
                        escape_tag(&data.sensor_id),
                        escape_tag(&data.location),
                        escape_tag(&data.process_stage),
                        data.temperature_celsius,
                        data.humidity_percent,
                        parsed_ts_influx
                    );

                    println!("📡 Line Protocol: {}", influx_line);

                    // Kirim data ke InfluxDB dengan timeout 15 detik
                    let influx_send_result = timeout(Duration::from_secs(15), async {
                        http_client.post(&influx_url)
                            .header("Authorization", format!("Token {}", influx_token))
                            .header("Content-Type", "text/plain; charset=utf-8")
                            .body(influx_line.clone())
                            .send()
                            .await
                    }).await;

                    match influx_send_result {
                        Ok(Ok(resp)) if resp.status().is_success() => {
                            println!("[{}] ✅ Data dikirim ke InfluxDB", timestamp_log);
                            writeln!(log_file, "[{}] Data dikirim ke InfluxDB: {}", timestamp_log, influx_line)?;
                        },
                        Ok(Ok(resp)) => {
                            let status = resp.status();
                            let body = resp.text().await.unwrap_or_default();
                            println!("[{}] ⚠️ Gagal kirim ke InfluxDB: {} - {}", timestamp_log, status, body);
                            writeln!(log_file, "[{}] Gagal kirim ke InfluxDB: {} - {}", timestamp_log, status, body)?;
                        },
                        Ok(Err(e)) => {
                            println!("[{}] ❌ HTTP Error ke InfluxDB: {}", timestamp_log, e);
                            writeln!(log_file, "[{}] HTTP Error ke InfluxDB: {}", timestamp_log, e)?;
                        },
                        Err(_) => { 
                            println!("[{}] ❌ Timeout (15s) saat kirim ke InfluxDB.", timestamp_log);
                            writeln!(log_file, "[{}] Timeout (15s) saat kirim ke InfluxDB.", timestamp_log)?;
                        }
                    }

                    // --- KIRIM DATA KE EVM ---
                    if let Err(e) = send_data_to_evm(
                        &contract,
                        &data,
                        &mut log_file
                    ).await {
                        println!("[{}] ❌ Gagal mengirim data ke EVM: {}", timestamp_log, e);
                        writeln!(log_file, "[{}] Gagal mengirim data ke EVM: {}", timestamp_log, e)?;
                    }

                    // <-- BAGIAN INI DIHAPUS (Tidak perlu kirim balik ke klien sensor)
                    // let json_line = format!("{}\n", line);
                    // let write_back_result = timeout(Duration::from_secs(5), async {
                    //     writer.write_all(json_line.as_bytes()).await?;
                    //     writer.flush().await?; 
                    //     Ok::<(), tokio::io::Error>(())
                    // }).await;

                    // match write_back_result {
                    //     Ok(Ok(_)) => {
                    //         println!("[{}] ✅ Data dikirim balik ke client: {}", timestamp_log, json_line.trim());
                    //     },
                    //     Ok(Err(e)) => {
                    //         println!("[{}] ⚠️ Gagal kirim data balik ke client: {}", timestamp_log, e);
                    //     },
                    //     Err(_) => {
                    //         println!("[{}] ⚠️ Timeout (5s) saat kirim balik data ke client.", timestamp_log);
                    //     }
                    // }
                },
                Err(e) => {
                    println!("[{}] ❌ Format JSON tidak valid dari {}: {}", timestamp_log, addr, e);
                    writeln!(log_file, "[{}] Format JSON tidak valid dari {}: {}", timestamp_log, addr, e)?;
                }
            }
        }
    }
    Ok(()) 
}


#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok(); 

    let influx_token = env::var("INFLUX_TOKEN")
        .expect("INFLUX_TOKEN tidak ditemukan di environment.");

    let evm_rpc_url = env::var("EVM_RPC_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8545".to_string()); 
    let evm_private_key_hex = env::var("EVM_PRIVATE_KEY")
        .expect("EVM_PRIVATE_KEY tidak ditemukan di environment (format hex tanpa 0x).");
    let evm_contract_address_str = env::var("EVM_CONTRACT_ADDRESS")
        .expect("EVM_CONTRACT_ADDRESS tidak ditemukan di environment.");

    let provider = Provider::<Http>::try_from(evm_rpc_url)?;
    let provider = Arc::new(provider); 

    let wallet: LocalWallet = evm_private_key_hex.parse::<LocalWallet>()?.with_chain_id(1337u64);
    
    let client_evm = Arc::new(SignerMiddleware::new(provider.clone(), wallet));

    let contract_address: Address = evm_contract_address_str.parse()?;
    
    let contract = SensorDataRecorder::new(contract_address, client_evm.clone());

    let influx_url = "http://localhost:8086/api/v2/write?org=ITS&bucket=ISI&precision=ns";

    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    let http_client = Client::new(); 

    println!("🚪 TCP Server listening on port 9000...");

    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("server.log")?;

    // <-- Kunci modifikasi: Inisialisasi broadcast channel
    let (tx_sensor_data, _) = broadcast::channel::<String>(16); 

    // Loop utama server untuk menerima koneksi baru
    loop {
        let (socket, addr) = listener.accept().await?; 
        let wib = chrono::FixedOffset::east_opt(7 * 3600).unwrap(); 
        let timestamp_main = wib.from_utc_datetime(&chrono::Utc::now().naive_utc()).to_rfc3339();
        println!("[{}] 🔌 Koneksi masuk dari {}", timestamp_main, addr);
        // writeln!(log_file, "[{}] Koneksi masuk dari {}", timestamp_main, addr)?; // Bisa dihapus agar log tidak duplikat

        // Kloning variabel yang diperlukan untuk spawned task agar dapat di-move
        let http_client_clone = http_client.clone();
        let influx_url_clone = influx_url.to_string();
        let influx_token_clone = influx_token.clone();
        let contract_clone = contract.clone(); 
        let log_file_clone = log_file.try_clone()?; 

        // <-- Kloning transmitter untuk digunakan di spawned task
        let tx_sensor_data_clone = tx_sensor_data.clone();

        // Spawn task baru untuk setiap koneksi klien agar dapat ditangani secara konkruen
        tokio::spawn(async move {
            // Panggil fungsi handle_connection untuk menangani logika klien
            if let Err(e) = handle_connection(
                socket,
                addr,
                http_client_clone,
                influx_url_clone,
                influx_token_clone,
                contract_clone,
                log_file_clone,
                tx_sensor_data_clone,
            ).await {
                eprintln!("[{}] Error handling client {}: {}", 
                          chrono::FixedOffset::east_opt(7 * 3600).unwrap().from_utc_datetime(&chrono::Utc::now().naive_utc()).to_rfc3339(), addr, e);
            }
        });
    }
}