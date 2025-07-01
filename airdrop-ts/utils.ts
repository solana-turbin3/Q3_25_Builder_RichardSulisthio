import { Keypair } from "@solana/web3.js";
import bs58 from "bs58";
import * as prompt from "prompt-sync";
import * as fs from "fs";

const input = prompt.default();

export function base58toWallet(base58?: string) {
  if (!base58) {
    base58 = input("Enter base58 private key: ");
  }
  const secretKey = bs58.decode(base58);
  return secretKey;
}

export function walletToBase58(secretKeyArray?: string) {
  if (!secretKeyArray) {
    secretKeyArray = input("Enter secret key array (e.g., [1,2,3,...]): ");
  }
  const secretKey = new Uint8Array(JSON.parse(secretKeyArray));
  const wallet = Keypair.fromSecretKey(secretKey);
  return bs58.encode(wallet.secretKey);
}

export function walletToBase58FromFile(filePath?: string) {
  if (!filePath) {
    filePath = input("Enter file path: ");
  }
  const secretKeyArray = fs.readFileSync(filePath, "utf8");
  return walletToBase58(secretKeyArray);
}

// CLI functionality
if (require.main === module) {
  console.log("Wallet Utility CLI");
  console.log("1. Convert base58 to wallet");
  console.log("2. Convert wallet to base58");
  console.log("3. Convert wallet to base58 from file");

  const choice = input("Choose an option (1, 2, or 3): ");

  if (choice === "1") {
    const secretKey = base58toWallet();
    console.log(`Secret Key: ${secretKey}`);
  } else if (choice === "2") {
    const base58 = walletToBase58();
    console.log(`Base58 Private Key: ${base58}`);
  } else if (choice === "3") {
    const base58 = walletToBase58FromFile();
    console.log(`Base58 Private Key: ${base58}`);
  } else {
    console.log("Invalid choice");
  }
}
