// TUGAS_ISI1/smartcontract/scripts/deploy.js
// Hapus baris ini: const { run } = require("hardhat");
const copyAbi = require("./copy-abi"); // <-- Import script copy-abi.js

async function main() {
  const SensorDataRecorder = await ethers.getContractFactory("SensorDataRecorder");
  const sensorDataRecorder = await SensorDataRecorder.deploy(); 

  await sensorDataRecorder.waitForDeployment();

  console.log("SensorDataRecorder deployed to:", sensorDataRecorder.target);

  // Panggil fungsi copyAbi dari script yang diimpor
  console.log("Copying ABI to dApp project...");
  await copyAbi.main(); // <-- Panggil fungsi main dari copy-abi.js
  console.log("ABI copied successfully.");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });