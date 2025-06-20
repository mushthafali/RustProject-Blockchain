use tokio_modbus::{client::rtu, prelude::*};
use tokio_serial::{SerialPortBuilderExt, Parity, StopBits, DataBits};
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt; // Sudah diperbaiki
use serde::Serialize;
use chrono::{Utc, TimeZone};
use std::error::Error;
use tokio::time::{sleep, Duration, timeout};
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Serialize, Debug)]
struct SensorData {
    timestamp: String,
    sensor_id: String,
    location: String,
    process_stage:String,
    temperature_celsius: f32,
    humidity_percent: f32,
}

// Tambahkan timeout untuk operasi baca sensor
async fn read_sensor(slave: u8) -> Result<Vec<u16>, Box<dyn Error>> {
    let builder = tokio_serial::new("/dev/ttyUSB0", 9600)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .data_bits(DataBits::Eight)
        .timeout(std::time::Duration::from_secs(2)); // Timeout untuk open_native_async

    // Tambahkan timeout untuk seluruh proses koneksi Modbus dan pembacaan
    let result = timeout(Duration::from_secs(10), async { // Timeout 10 detik untuk Modbus
        let port = builder.open_native_async()?;
        let mut ctx = rtu::connect_slave(port, Slave(slave)).await?;
        let response = ctx.read_input_registers(1, 2).await?;
        Ok(response)
    }).await;

    match result {
        Ok(Ok(response)) => Ok(response), // Berhasil mendapatkan respons
        Ok(Err(e)) => Err(e), // Error dari operasi Modbus itu sendiri
        Err(_) => Err("Timeout saat baca sensor Modbus".into()), // Timeout dari tokio::time::timeout
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("client.log")?;

    println!("Program dimulai pada: {}", Utc::now().to_rfc3339());

    // --- KUNCI MODIFIKASI: Deklarasi variabel untuk koneksi TCP di luar loop ---
    let mut tcp_stream_option: Option<TcpStream> = None; 

    loop {
        // Coba untuk membentuk atau menyambungkan kembali koneksi jika tidak ada koneksi aktif
        if tcp_stream_option.is_none() {
            println!("[{}] Mencoba menyambung ke TCP server...", Utc::now().to_rfc3339());
            writeln!(log_file, "[{}] Mencoba menyambung ke TCP server...", Utc::now().to_rfc3339())?;

            match timeout(Duration::from_secs(5), TcpStream::connect("127.0.0.1:9000")).await {
                Ok(Ok(mut stream)) => { // <-- PERBAIKAN: Tambahkan 'mut' di sini
                    println!("[{}] ✅ Terhubung ke TCP server!", Utc::now().to_rfc3339());
                    writeln!(log_file, "[{}] ✅ Terhubung ke TCP server!", Utc::now().to_rfc3339())?;
                    
                    // Kirim identifikasi sebagai klien sensor (PENTING untuk server)
                    if let Err(e) = stream.write_all(b"SENSOR_CLIENT\n").await {
                        println!("[{}] ❌ Gagal kirim identifikasi client: {}", Utc::now().to_rfc3339(), e);
                        writeln!(log_file, "[{}] ❌ Gagal kirim identifikasi client: {}", Utc::now().to_rfc3339(), e)?;
                        sleep(Duration::from_secs(2)).await;
                        continue; 
                    }
                    if let Err(e) = stream.flush().await {
                        println!("[{}] ❌ Gagal flush identifikasi: {}", Utc::now().to_rfc3339(), e);
                        writeln!(log_file, "[{}] ❌ Gagal flush identifikasi: {}", Utc::now().to_rfc3339(), e)?;
                        sleep(Duration::from_secs(2)).await;
                        continue;
                    }
                    
                    tcp_stream_option = Some(stream); 
                },
                Ok(Err(e)) => {
                    println!("[{}] ❌ Gagal konek ke TCP server: {}", Utc::now().to_rfc3339(), e);
                    writeln!(log_file, "[{}] ❌ Gagal konek ke TCP server: {}", Utc::now().to_rfc3339(), e)?;
                    sleep(Duration::from_secs(5)).await;
                    continue; 
                },
                Err(_) => { 
                    println!("[{}] ❌ Timeout saat konek ke TCP server", Utc::now().to_rfc3339());
                    writeln!(log_file, "[{}] ❌ Timeout saat konek ke TCP server", Utc::now().to_rfc3339())?;
                    sleep(Duration::from_secs(5)).await;
                    continue; 
                }
            }
        }
        
        let wib = chrono::offset::FixedOffset::east_opt(7 * 3600).ok_or("Invalid timezone")?;
        let timestamp = wib.from_utc_datetime(&Utc::now().naive_utc()).to_rfc3339();

        match read_sensor(1).await {
            Ok(response) if response.len() == 2 => {
                let temp = response[0] as f32 / 100.0;
                let rh = response[1] as f32 / 100.0;

                if temp < -40.0 || temp > 125.0 || rh < 0.0 || rh > 100.0 {
                    println!("[{}] ⚠️ Bacaan tidak valid: Suhu {:.1} °C, Kelembapan {:.1} %", timestamp, temp, rh);
                    writeln!(log_file, "[{}] Bacaan tidak valid: Suhu {:.1} °C, Kelembapan {:.1} %", timestamp, temp, rh)?;
                } else {
                    println!("[{}] 📡 Suhu: {:.1} °C | Kelembapan: {:.1} %", timestamp, temp, rh);
                    writeln!(log_file, "[{}] Suhu: {:.1} °C | Kelembapan: {:.1} %", timestamp, temp, rh)?;

                    let data = SensorData {
                        timestamp: timestamp.clone(),
                        sensor_id: "SHT20-PascaPanen-001".into(),
                        location: "Gudang Fermentasi 1".into(),
                        process_stage: "Fermentasi".into(),
                        temperature_celsius: temp,
                        humidity_percent: rh,
                    };
                    let json = serde_json::to_string(&data)?;
                    println!("[{}] 📤 Mengirim: {}", timestamp, json);
                    writeln!(log_file, "[{}] Mengirim: {}", timestamp, json)?;

                    if let Some(stream) = tcp_stream_option.as_mut() {
                        let send_result = timeout(Duration::from_secs(5), async {
                            stream.write_all(json.as_bytes()).await?;
                            stream.write_all(b"\n").await?;
                            stream.flush().await?;
                            Ok::<(), Box<dyn Error>>(())
                        }).await;

                        match send_result {
                            Ok(Ok(_)) => {
                                println!("[{}] ✅ Data dikirim ke TCP server", timestamp);
                                writeln!(log_file, "[{}] Data dikirim ke TCP server", timestamp)?;
                            },
                            Ok(Err(e)) => {
                                println!("[{}] ❌ Gagal kirim data ke TCP server (Error internal): {}", timestamp, e);
                                writeln!(log_file, "[{}] Gagal kirim data ke TCP server (Error internal): {}", timestamp, e)?;
                                tcp_stream_option = None; 
                            },
                            Err(_) => { 
                                println!("[{}] ❌ Timeout saat kirim data ke TCP server", timestamp);
                                writeln!(log_file, "[{}] ❌ Timeout saat kirim data ke TCP server", timestamp)?;
                                tcp_stream_option = None; 
                            }
                        }
                    } else {
                        println!("[{}] ⚠️ Tidak ada koneksi TCP aktif untuk mengirim data. Akan mencoba menyambung lagi.", timestamp);
                        writeln!(log_file, "[{}] Tidak ada koneksi TCP aktif untuk mengirim data. Akan mencoba menyambung lagi.", timestamp)?;
                        tcp_stream_option = None; 
                    }
                }
            },
            Ok(other) => {
                println!("[{}] ⚠️ Data tidak lengkap dari sensor: {:?}", timestamp, other);
                writeln!(log_file, "[{}] Data tidak lengkap dari sensor: {:?}", timestamp, other)?;
            },
            Err(e) => {
                println!("[{}] ❌ Gagal baca sensor: {}", timestamp, e);
                writeln!(log_file, "[{}] Gagal baca sensor: {}", timestamp, e)?;
            }
        }

        sleep(Duration::from_secs(2)).await; 
    }
}