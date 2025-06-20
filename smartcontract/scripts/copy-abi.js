const fs = require('fs');
const path = require('path');

async function main() {
    // Path ke ABI yang sudah dikompilasi oleh Hardhat
    const sourcePath = path.join(__dirname, '../artifacts/contracts/SensorData.sol/SensorDataRecorder.json');
    // Path tujuan di folder src dApp Anda
    // INI YANG PENTING: PASTIKAN INI ADALAH 'dapp' (huruf kecil)
    const destinationDir = path.join(__dirname, '../../dapp/src/contracts/'); 
    const destinationPath = path.join(destinationDir, 'SensorDataRecorder.json');

    // Buat folder tujuan jika belum ada
    if (!fs.existsSync(destinationDir)) {
        fs.mkdirSync(destinationDir, { recursive: true });
    }

    // Salin file ABI
    fs.copyFileSync(sourcePath, destinationPath);
    console.log(`ABI copied from ${sourcePath} to ${destinationPath}`);
}

// Export fungsi main agar bisa dipanggil dari deploy.js
module.exports.main = main;