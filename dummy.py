import socket
import json
import time
from datetime import datetime

def send_dummy_data():
    while True:
        try:
            with socket.create_connection(("127.0.0.1", 9000)) as sock:
                print("✅ Connected to TCP Server")

                while True:
                    # Format waktu RFC3339
                    now = datetime.now().astimezone().isoformat()

                    data = {
                        "timestamp": now,
                        "sensor_id": "sensor001",
                        "location": "lab1",
                        "process_stage": "initial",
                        "temperature_celsius": 26.3,
                        "humidity_percent": 47.8
                    }

                    json_str = json.dumps(data) + "\n"
                    sock.sendall(json_str.encode())
                    print("📤 Sent:", json_str.strip())

                    time.sleep(5)  # Kirim tiap 5 detik
        except Exception as e:
            print(f"❌ Connection failed: {e}. Reconnecting in 5s...")
            time.sleep(5)

if __name__ == "__main__":
    send_dummy_data()
