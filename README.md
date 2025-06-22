# 🦀 RustProject-Blockchain

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

| Komponen       | Teknologi                             |
| -------------- | ------------------------------------- |
| Backend Server | Rust, Tokio, Serde, Reqwest           |
| Database       | InfluxDB v2                           |
| Visualisasi    | Grafana, PyQtGraph, Recharts          |
| Blockchain     | Solidity, Ganache, ethers.js, ABI     |
| GUI            | PyQt6                                 |
| Frontend Web   | React, TypeScript, Web3.js, PapaParse |
| Komunikasi     | TCP Socket                            |

---

## 📦 Struktur Direktori

```
RustProject-Blockchain/
├── server/             # Rust TCP server
├── sht20/             # Client Rust pengirim data sensor
├── qt_gui/             # PyQt GUI real-time
├── dapp/               # React dashboard Web3
├── smartcontract/      # Solidity contract dan ABI
├── .env                # Konfigurasi koneksi Ethereum
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
npm install
npm start
```

---

### 👛 InfluxDB & Grafana (Opsional)

Jika menggunakan Docker, jalankan:

```bash
docker-compose up -d
```

- InfluxDB: [http://localhost:8086](http://localhost:8086)
- Grafana:  [http://localhost:3001](http://localhost:3001)

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

## 💡 Contoh Output

**GUI PyQt:**

- ✔️ Suhu dan kelembapan real-time
- ✔️ Indikator “data transferred successfully”

**React Dashboard:**

- 📈 Grafik sensor historis
- 📥 Tombol ekspor CSV

**Grafana Panel:**

- 📊 Visualisasi time-series dari InfluxDB

---

## 🔖 Lisensi

Proyek ini dilisensikan di bawah MIT License.

---

## 🤝 Kontribusi

Pull request dan masukan sangat diterima. Silakan fork repo ini dan mulai kontribusi!

---

## ✨ Penulis

- **Mushthafa Ali Akbar**\
- **Axel Fitra Ananda**\
- **Lu'Lu' Rusyida**\
  Teknik Instrumentasi ITS
  [GitHub](https://github.com/mushthafali)

