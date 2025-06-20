// TUGAS_ISI1/smartcontract/hardhat.config.js
require("@nomicfoundation/hardhat-toolbox");

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: "0.8.20", // Harus cocok dengan `pragma` di SensorData.sol
  networks: {
    ganache: {
      url: "http://127.0.0.1:8545", // SESUAIKAN DENGAN PORT GANACHE ANDA
    }
  }
};
