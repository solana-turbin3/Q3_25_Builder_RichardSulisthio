import { Keypair } from "@solana/web3.js";
import * as fs from "fs";

const kp = Keypair.generate();

console.log(
  `You have generated a new Solana Wallet: ${kp.publicKey.toBase58()}`,
);

// save to wallet.json in ignored out folder for safety
fs.writeFileSync("out/dev-wallet.json", `[${kp.secretKey}]`);
console.log("Wallet saved to wallet.json");
