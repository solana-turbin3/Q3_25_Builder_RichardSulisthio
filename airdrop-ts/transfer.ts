import {
  Transaction,
  SystemProgram,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  sendAndConfirmTransaction,
  PublicKey,
} from "@solana/web3.js";

import wallet from "./out/dev-wallet.json";

// We're going to import our keypair from the wallet file, this is the wallet that will send the transaction
const from = Keypair.fromSecretKey(new Uint8Array(wallet));
const to = new PublicKey("3yfEkerzEcDvn1rMmuT3JZ5FbbCzYnbBma8Y1knHEUiT");

const connection = new Connection("https://api.devnet.solana.com");

(async () => {
  try {
    // get balance of the from address
    const balance = await connection.getBalance(from.publicKey);

    // create a new transaction
    const transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance,
      }),
    );

    // get the latest blockhash
    transaction.recentBlockhash = (
      await connection.getLatestBlockhash("confirmed")
    ).blockhash;

    // fee payer should be from
    transaction.feePayer = from.publicKey;

    // Calculate exact fee rate to transfer entire SOL amount out of account minus fees
    const fee =
      (
        await connection.getFeeForMessage(
          transaction.compileMessage(),
          "confirmed",
        )
      ).value || 0;

    // Remove our transfer instruction to replace it
    transaction.instructions.pop();

    // Add a new transfer instruction with the exact fee rate
    transaction.add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance - fee,
      }),
    );

    const signature = await sendAndConfirmTransaction(connection, transaction, [
      from,
    ]);
    console.log(`Success! Check out your TX here:
https://explorer.solana.com/tx/${signature}?cluster=devnet`);
  } catch (error) {
    console.error("Oops, something went wrong:", error);
  }
})();
