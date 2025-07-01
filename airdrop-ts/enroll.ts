import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
import { IDL, Turbin3Prereq } from "./programs/Turbin3_prereq";
import wallet from "./out/Turbin3-wallet.json";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
const MPL_CORE_PROGRAM_ID = new PublicKey(
  "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d",
);

// get keypair
const t3UserAccountKeypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// create devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// create provider
const provider = new AnchorProvider(
  connection,
  new Wallet(t3UserAccountKeypair),
  {
    commitment: "confirmed",
  },
);

// create program
const program: Program<Turbin3Prereq> = new Program(IDL, provider);

const accountSeeds = [
  Buffer.from("prereqs"),
  t3UserAccountKeypair.publicKey.toBuffer(),
];

const [accountKey, _accountBump] = PublicKey.findProgramAddressSync(
  accountSeeds,
  program.programId,
);

// the collection to be minted
const mintCollection = new PublicKey(
  "5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2",
);

// generate a mint account for the asset
const mintTs = Keypair.generate();

// Execute the initialize transaction
(async () => {
  try {
    const txhash = await program.methods
      .initialize("metalboyrick")
      .accountsPartial({
        user: t3UserAccountKeypair.publicKey,
        account: accountKey,
        system_program: SYSTEM_PROGRAM_ID,
      })
      .signers([t3UserAccountKeypair])
      .rpc();
    console.log(`Success! Check out your TX here:
https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();

// find the authority
const [authorityKey, _authorityBump] = PublicKey.findProgramAddressSync(
  [
    Buffer.from([99, 111, 108, 108, 101, 99, 116, 105, 111, 110]),
    mintCollection.toBuffer(),
  ],
  program.programId,
);
console.log("Authority Key:", authorityKey.toBase58());

// Execute the submitTs transaction
(async () => {
  try {
    const txhash = await program.methods
      .submitTs()
      .accountsPartial({
        user: t3UserAccountKeypair.publicKey,
        account: accountKey,
        mint: mintTs.publicKey,
        collection: mintCollection,
        authority: authorityKey,
        mpl_core_program: MPL_CORE_PROGRAM_ID,
        system_program: SYSTEM_PROGRAM_ID,
      })
      .signers([t3UserAccountKeypair, mintTs])
      .rpc();
    console.log(`Success! Check out your TX here:
https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
