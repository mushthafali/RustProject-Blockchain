// TUGAS_ISI1/dApp/src/global.d.ts
// Ini mendeklarasikan properti 'ethereum' pada objek 'window' global
interface Window {
  // Menggunakan `any` untuk fleksibilitas. Jika Anda ingin lebih spesifik,
  // bisa menggunakan `import('ethers').Eip11993Provider;` jika `ethers` v6 diinstal.
  ethereum?: any; 
}