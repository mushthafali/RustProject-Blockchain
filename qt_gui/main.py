import json
import socket
import threading
from datetime import datetime

from PyQt6.QtWidgets import QWidget, QLabel, QVBoxLayout, QApplication
from PyQt6.QtCore import QTimer, Qt, pyqtSignal, QObject
import pyqtgraph as pg
import sys

# Kelas helper untuk memancarkan sinyal dari thread non-GUI ke thread GUI
class SignalEmitter(QObject):
    data_received_signal = pyqtSignal(str)
    # Sinyal untuk memperbarui status koneksi, agar aman dari thread lain
    connection_status_signal = pyqtSignal(str)

class SensorWindow(QWidget):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("🌡️ Sensor Monitor GUI")
        self.setGeometry(300, 200, 800, 500)

        # Inisialisasi emitter dan hubungkan sinyal-sinyalnya ke slot yang sesuai
        self.emitter = SignalEmitter()
        self.emitter.data_received_signal.connect(self.process_json)
        self.emitter.connection_status_signal.connect(self.update_status_label)

        self.init_ui() # Inisialisasi antarmuka pengguna
        self.start_tcp_thread() # Mulai thread untuk mendengarkan data TCP

    def init_ui(self):
        # Membuat label untuk menampilkan suhu dan kelembaban
        self.temp_label = QLabel("🌡️ Temperature: -- °C")
        self.hum_label = QLabel("💧 Humidity: -- %")
        self.status_label = QLabel("🔌 Status: Not connected")

        # Mengatur style untuk label-label
        for label in [self.temp_label, self.hum_label, self.status_label]:
            label.setStyleSheet("font-size: 18px; padding: 4px;")

        # Membuat widget plot menggunakan PyQtGraph
        self.graph = pg.PlotWidget(title="📈 Real-time Temperature & Humidity")
        self.graph.setBackground('w') # Latar belakang grafik putih
        self.graph.addLegend() # Menampilkan legenda
        self.graph.showGrid(x=True, y=True) # Menampilkan grid

        # Membuat "kurva" awal untuk suhu dan kelembaban di grafik
        self.temp_curve = self.graph.plot(pen=pg.mkPen('r', width=2), name="Temperature")
        self.hum_curve = self.graph.plot(pen=pg.mkPen('b', width=2), name="Humidity")

        # List untuk menyimpan data yang akan diplot
        self.temp_data = []
        self.hum_data = []
        self.time_data = [] # Menyimpan nilai numerik untuk sumbu X (indeks)

        # Mengatur layout utama aplikasi
        layout = QVBoxLayout()
        layout.addWidget(self.temp_label)
        layout.addWidget(self.hum_label)
        layout.addWidget(self.status_label)
        layout.addWidget(self.graph)
        self.setLayout(layout)

        # QTimer untuk memperbarui plot secara berkala (setiap 1 detik)
        self.timer = QTimer()
        self.timer.timeout.connect(self.update_plot)
        self.timer.start(1000) # Memanggil update_plot setiap 1000 ms (1 detik)

    def start_tcp_thread(self):
        # Membuat dan memulai thread terpisah untuk operasi TCP agar GUI tidak nge-hang
        thread = threading.Thread(target=self.tcp_listener, args=(self.emitter,), daemon=True)
        thread.start()

    def tcp_listener(self, emitter):
        try:
            # Mencoba membuat koneksi TCP ke server Rust
            sock = socket.create_connection(("127.0.0.1", 9000))
            # PENTING: Mengirim string identifikasi ke server
            sock.sendall(b"GUI_CLIENT\n")
            # Memancarkan sinyal untuk memperbarui status koneksi di GUI utama
            emitter.connection_status_signal.emit("✅ Connected to TCP Server")
            print("✅ Connected to TCP Server (from TCP thread)")

            buffer = "" # Buffer untuk menyimpan data yang mungkin belum lengkap
            while True:
                # Menerima data dari socket
                data = sock.recv(1024).decode('utf-8') # Decode data dengan encoding UTF-8
                if not data: # Jika tidak ada data, koneksi ditutup oleh server
                    print("❌ Connection closed by server (from TCP thread)")
                    emitter.connection_status_signal.emit("❌ Disconnected from Server")
                    break

                buffer += data # Tambahkan data yang diterima ke buffer
                # Memproses baris demi baris jika ada karakter newline
                while "\n" in buffer:
                    line, buffer = buffer.split("\n", 1) # Pisahkan baris pertama dan sisa buffer
                    if line.strip(): # Pastikan baris tidak kosong setelah di-strip
                        # Memancarkan sinyal dengan data JSON yang diterima ke thread GUI utama
                        emitter.data_received_signal.emit(line.strip())

        except Exception as e:
            # Menangani error koneksi TCP
            print(f"❌ TCP connection error (from TCP thread): {e}")
            emitter.connection_status_signal.emit(f"❌ Connection failed: {e}")

    def update_status_label(self, message):
        # Slot ini dipanggil oleh sinyal connection_status_signal untuk memperbarui status_label
        # Ini memastikan pembaruan UI dilakukan di thread GUI utama
        self.status_label.setText(message)

    def process_json(self, json_str):
        # Slot ini akan dijalankan di thread utama GUI karena dipicu oleh sinyal data_received_signal
        try:
            # print(f"📥 Received from server (GUI thread): {json_str}") # Hapus komentar untuk debugging detail
            data = json.loads(json_str) # Menguraikan string JSON menjadi objek Python

            # Mengambil nilai suhu dan kelembaban dari data JSON
            temp = data["temperature_celsius"]
            hum = data["humidity_percent"]

            # Memperbarui label suhu dan kelembaban di GUI
            self.temp_label.setText(f"🌡️ Temperature: {temp:.2f} °C")
            self.hum_label.setText(f"💧 Humidity: {hum:.2f} %")
            self.status_label.setText("✅ Data Transferred Successfully")

            # Menambahkan titik data baru untuk grafik
            current_x = len(self.time_data) # Gunakan indeks sebagai nilai X
            self.time_data.append(current_x)
            self.temp_data.append(temp)
            self.hum_data.append(hum)

            # Mempertahankan hanya 50 titik data terakhir untuk menjaga kinerja grafik
            if len(self.time_data) > 50:
                self.time_data.pop(0) # Hapus titik data tertua
                self.temp_data.pop(0)
                self.hum_data.pop(0)
                # Penting: Re-index nilai x setelah pop agar tetap berurutan untuk plotting
                self.time_data = list(range(len(self.time_data)))


        except json.JSONDecodeError as e:
            # Menangani error jika string JSON tidak valid
            print(f"❌ Failed to parse JSON (GUI thread): {json_str} - {e}")
            self.status_label.setText(f"⚠️ Invalid JSON format")
        except KeyError as e:
            # Menangani error jika kunci yang diharapkan tidak ditemukan dalam JSON
            print(f"❌ Missing key in JSON (GUI thread): {e} in {json_str}")
            self.status_label.setText(f"⚠️ Missing data: {e}")
        except Exception as e:
            # Menangani error umum lainnya saat memproses data
            print(f"❌ Error processing data (GUI thread): {e}")
            self.status_label.setText(f"⚠️ Processing Error")

    def update_plot(self):
        # Fungsi ini dipanggil secara berkala oleh QTimer (di thread GUI utama)
        if self.time_data:
            # Re-index nilai x untuk memastikan mereka berurutan untuk plotting
            x_values = list(range(len(self.time_data)))
            # Memperbarui data untuk kurva suhu dan kelembaban
            self.temp_curve.setData(x_values, self.temp_data)
            self.hum_curve.setData(x_values, self.hum_data)

            # Menyesuaikan rentang sumbu X agar selalu menampilkan 50 titik data terakhir
            self.graph.setXRange(max(0, len(self.time_data) - 50), len(self.time_data))
            # Opsi: Mengatur rentang sumbu Y secara otomatis
            # self.graph.autoRange(y=True)


if __name__ == "__main__":
    # Membuat objek aplikasi Qt
    app = QApplication(sys.argv)
    # Membuat instance jendela sensor
    window = SensorWindow()
    # Menampilkan jendela
    window.show()
    # Memulai event loop aplikasi Qt
    sys.exit(app.exec())