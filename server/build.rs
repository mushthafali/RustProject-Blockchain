// TUGAS_ISI1/Server/build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // PERBAIKI PATH DI SINI!
    println!("cargo:rerun-if-changed=../smartcontract/artifacts/contracts/SensorData.sol/SensorDataRecorder.json");

    ethers_contract_abigen::Abigen::new(
        "SensorDataRecorder", 
        "../smartcontract/artifacts/contracts/SensorData.sol/SensorDataRecorder.json", // <-- UBAH 'sensor-contract-evm' menjadi 'smartcontract'
    )?
    .generate()?
    .write_to_file("src/abi.rs")?; 

    Ok(())
}