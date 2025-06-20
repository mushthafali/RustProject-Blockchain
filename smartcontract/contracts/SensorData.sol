// TUGAS_ISI1/smartcontract/contracts/SensorData.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20; // Menggunakan versi Solidity terbaru

contract SensorDataRecorder {
    // Definisi struktur data sensor
    struct SensorReading {
        uint64 timestampUnixNano;      // Timestamp dalam nanodetik (uint64)
        string sensorId;
        string location;
        string processStage;
        int32 temperatureCelsiusScaled; // Suhu (dikalikan 100 untuk 2 desimal)
        uint32 humidityPercentScaled;    // Kelembaban (dikalikan 100 untuk 2 desimal)
        address recorder;               // Alamat yang merekam data
    }

    // Array untuk menyimpan semua pembacaan sensor
    // Peringatan: Penyimpanan array di blockchain itu mahal dan tidak efisien untuk data besar.
    // Untuk prototipe/pengujian ini ok. Untuk produksi, pertimbangkan alternatif (seperti mapping, data off-chain).
    SensorReading[] public allSensorReadings;

    // Event untuk memudahkan pelacakan data yang direkam di log blockchain
    event DataRecorded(
        uint64 indexed timestampUnixNano,
        string indexed sensorId,
        int32 temperatureCelsiusScaled,
        uint32 humidityPercentScaled,
        address recorder
    );

    // Fungsi untuk merekam data sensor baru
    function recordSensorData(
        uint64 _timestampUnixNano,
        string memory _sensorId,
        string memory _location,
        string memory _processStage,
        int32 _temperatureCelsiusScaled,
        uint32 _humidityPercentScaled
    ) public {
        // Validasi nilai sensor
        require(_temperatureCelsiusScaled >= -4000 && _temperatureCelsiusScaled <= 12500, "Invalid temperature range (-40 to 125 C scaled)");
        require(_humidityPercentScaled >= 0 && _humidityPercentScaled <= 10000, "Invalid humidity range (0 to 100 % scaled)");

        SensorReading memory newReading = SensorReading({
            timestampUnixNano: _timestampUnixNano,
            sensorId: _sensorId,
            location: _location,
            processStage: _processStage,
            temperatureCelsiusScaled: _temperatureCelsiusScaled,
            humidityPercentScaled: _humidityPercentScaled,
            recorder: msg.sender // `msg.sender` adalah alamat yang memanggil fungsi ini
        });

        allSensorReadings.push(newReading);

        // Emit event setelah data direkam
        emit DataRecorded(
            _timestampUnixNano,
            _sensorId,
            _temperatureCelsiusScaled,
            _humidityPercentScaled,
            msg.sender
        );
    }

    // Fungsi untuk mendapatkan semua data sensor (read-only)
    // Peringatan: Untuk array sangat besar, ini akan menghabiskan banyak gas dan bisa gagal.
    // Untuk pengembangan ok. Untuk produksi, pertimbangkan fungsi paginated.
    function getAllSensorData() public view returns (SensorReading[] memory) {
        return allSensorReadings;
    }

    // Fungsi untuk mendapatkan jumlah total data sensor
    function getSensorDataCount() public view returns (uint256) {
        return allSensorReadings.length;
    }
}
