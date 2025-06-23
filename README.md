# 🦀 RustProject-Blockchain-DApp Web3

RustProject-Blockchain adalah sistem monitoring data sensor suhu dan kelembapan berbasis **Rust**, yang terhubung dengan **InfluxDB**, **Grafana**, **smart contract Ethereum**, dan antarmuka GUI **PyQt** serta dashboard **Web3 (React)**. Proyek ini dirancang untuk memberikan solusi **IoT + Blockchain + Real-time Dashboard** dalam satu kesatuan sistem.

---

## 🚧 Fitur Utama

- 🌐 TCP Server (Rust): Menerima data JSON dari sensor
- 🧐 Penyimpanan data ke InfluxDB v2 untuk visualisasi
- 🔐 Pengiriman data ke smart contract Ethereum menggunakan ABI dan Web3
- 📈 Dashboard Web3 (React) dengan grafik dan fitur ekspor CSV
- 💻 GUI PyQt real-time dengan chart suhu dan kelembapan
- 📊 Grafana Dashboard terhubung ke InfluxDB

---

## ⚙️ Teknologi yang Digunakan

| **Kategori**   | **Teknologi**                                               | **Penjelasan**                                                                                                                                                                                                                                                                                                                           |
| -------------- | ----------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Backend Server | `Rust`, `Tokio`, `Serde`, `Reqwest`                         | - Rust adalah bahasa inti untuk server TCP.- Tokio digunakan untuk asynchronous I/O (agar server bisa menangani banyak koneksi).- Serde digunakan untuk parsing/serialisasi data JSON dari sensor.- Reqwest dipakai untuk HTTP client, contohnya untuk kirim data ke InfluxDB v2 atau Web3 API.                                                 |
| Sensor Client  | `Rust + JSON via TCP`                                       | Program Rust yang berjalan sebagai pengirim data dari sensor ke server menggunakan koneksi TCP dalam format JSON.                                                                                                                                                                                                                               |
| Komunikasi     | `TCP Socket`                                                | Protokol komunikasi utama antara sensor, server, dan GUI (menggunakan socket TCP).                                                                                                                                                                                                                                                              |
| Database       | `InfluxDB v2`                                               | Time-series database untuk menyimpan data suhu dan kelembapan berdasarkan timestamp.                                                                                                                                                                                                                                                            |
| Visualisasi    | `Grafana`, `PyQtGraph`, `Recharts`                          | - Grafana untuk panel data dari InfluxDB.- PyQtGraph digunakan dalam GUI Python untuk menampilkan grafik real-time.- Recharts adalah library grafik pada React untuk web dashboard.                                                                                                                                                             |
| GUI Realtime   | `PyQt6`                                                     | Framework GUI berbasis Python, digunakan untuk membuat antarmuka visual real-time.                                                                                                                                                                                                                                                              |
| Frontend Web   | `React`, `TypeScript`, `PapaParse`, `Recharts`, `ethers.js` | - React adalah framework frontend.- TypeScript memberikan type safety.- PapaParse untuk mengubah data menjadi CSV.- Recharts menampilkan data dalam grafik.- ethers.js digunakan untuk membaca data dari smart contract.                                                                                                                        |
| Blockchain     | `Solidity`, `Hardhat`, `Ganache`, `ethers.js`, `ABI`        | - Solidity adalah bahasa untuk menulis smart contract.- Hardhat digunakan untuk compile, test, dan deploy smart contract.- Ganache adalah Ethereum node lokal untuk testing.- ethers.js untuk interaksi dengan smart contract via JavaScript.- ABI (Application Binary Interface) dibutuhkan agar Rust/React bisa bicara dengan smart contract. |

---

## 📦 Struktur Direktori

```
RustProject-Blockchain/
├── server/             # Rust TCP server
├── sht20/             # Client Rust pengirim data sensor
├── qt_gui/             # PyQt GUI real-time
├── dapp/               # React dashboard Web3
├── smartcontract/      # Solidity contract dan ABI
└── README.md
```

---

## 🚀 Langkah-Langkah Menjalankan Program (Versi Lengkap)

Berikut adalah urutan lengkap menjalankan sistem dari awal:

### 🔧 1. Buka Ubuntu & Aktifkan USB

- Masuk ke Ubuntu (jika via VM)
- Klik menu `Devices > USB > centang QinHeng Electronics HL-340`
- Buka terminal: `Ctrl + Alt + T`

### 📆 2. Jalankan Ganache (Smart Contract)

```bash
cd ~/tugasisi/smartcontract
ganache --port 8545 --db ./ganache-data --deterministic
```

### 💻 3. Buka VSCode dan Terminal-Terminal

- Buka folder proyek di **VSCode**
- Buka terminal-terpisah berikut untuk menjalankan komponen-komponen utama:

#### 🟡 Terminal `server` (Rust TCP Server)

```bash
cd server
cargo run
```

#### 🔵 Terminal `sensor` (Rust Client Pengirim Data)

```bash
cd sensor
cargo run
```

#### 🟢 Terminal `qt_gui` (GUI PyQt)

```bash
cd qt_gui
source venv/bin/activate
python main.py
```

#### 🌐 Terminal `dapp` (React Web3 Dashboard)

```bash
cd dapp
npm start
```

---

## 🍅 InfluxDB, Grafana & DApp Address

| Komponen | Alamat                                      |
|----------|---------------------------------------------|
| InfluxDB | [http://localhost:8086](http://localhost:8086) |
| Grafana  | [http://localhost:3001](http://localhost:3000) |
| DApp     | [http://localhost:3000](http://localhost:3001) |


---

## 📀 Urutan Eksekusi Ringkas

| Langkah | Komponen       | Perintah Singkat                                          |
| ------- | -------------- | --------------------------------------------------------- |
| 1       | Ganache        | `ganache --port 8545 --db ...`                            |
| 2       | TCP Server     | `cd server && cargo run`                                  |
| 3       | Client Sensor  | `cd sensor && cargo run`                                  |
| 4       | GUI PyQt       | `cd qt_gui && source venv/bin/activate && python main.py` |
| 5       | Web3 Dashboard | `cd dapp && npm start`                                    |

---

## 💡 Output

**GUI PyQt:**

- ✔️ Suhu dan kelembapan real-time
- ✔️ Indikator “data transferred successfully”

**React Dashboard:**

- 📈 Grafik sensor historis
- 📥 Tombol ekspor CSV

**Grafana Panel:**

- 📊 Visualisasi time-series dari InfluxDB

---

## ✨ Author

- **Mushthafa Ali Akbar**
- **Axel Fitra Ananda**
- **Lu'Lu' Rusyida Hamudya**\
  Teknik Instrumentasi ITS
  [GitHub](https://github.com/mushthafali)

