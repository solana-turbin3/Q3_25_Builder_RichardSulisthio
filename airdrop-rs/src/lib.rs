#[cfg(test)]
mod tests {
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey, system_instruction::transfer, hash::hash, instruction::{AccountMeta, Instruction}};
    use solana_sdk::{
        signature::{Keypair, read_keypair_file},
        signer::Signer,
        message::Message,
        transaction::Transaction,
    };
    use bs58;
    use std::io::{self, BufRead};
    use std::str::FromStr;

    const RPC_URL: &str = "https://api.devnet.solana.com";
    const TURBIN3_PUBLIC_KEY: &str = "3yfEkerzEcDvn1rMmuT3JZ5FbbCzYnbBma8Y1knHEUiT";
    const TURBIN3_PREREQ_PROGRAM: &str = "TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM";
    const COLLECTION: &str = "5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2";
    const MPL_CORE_PROGRAM: &str = "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d";
    const SYSTEM_PROGRAM: &str = "11111111111111111111111111111111";


    #[test]
    fn keygen() {
        let keypair = Keypair::new();
        let pubkey = keypair.pubkey();
        println!("Public key: {}", pubkey.to_string());
        println!("Private key: {:?}", keypair.to_bytes());
    }

    #[test]
    fn airdrop() {
        let client = RpcClient::new(RPC_URL);
        let keypair = read_keypair_file("dev-wallet.json").expect("Failed to read keypair file");
        let balance = client.get_balance(&keypair.pubkey()).unwrap();
        println!("Balance Before: {}", balance);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(sig) => {
                println!("Success! Check your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
            }
            Err(err) => {
                println!("Airdrop failed: {}", err);
            }
        }

        let balance = client.get_balance(&keypair.pubkey()).unwrap();
        println!("Balance After: {}", balance);
    }

    #[test]
    fn transfer_sol() {
        let client = RpcClient::new(RPC_URL);
        let keypair = read_keypair_file("dev-wallet.json").expect("Failed to read keypair file");
        let balance = client.get_balance(&keypair.pubkey()).unwrap();
        println!("Balance Before: {}", balance);

       let pubkey = keypair.pubkey();
        let message_bytes = b"I verify my Solana Keypair!";
        let sig = keypair.sign_message(message_bytes);
        let sig_hashed = hash(sig.as_ref());
        println!("Signature: {:?}", sig_hashed);

        // this is a test to verify the signature with you public key
        // match syntax DOES NOT THROW, 
        match sig.verify(&pubkey.to_bytes(), message_bytes) {
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }


        let to_pubkey = Pubkey::from_str(TURBIN3_PUBLIC_KEY).unwrap();

        let recent_blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // let transaction = Transaction::new_signed_with_payer(
        //     &[transfer(&keypair.pubkey(), &to_pubkey, 100_000_000)],
        //     Some(&keypair.pubkey()),
        //     &vec![&keypair],
        //     recent_blockhash,
        // );

        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        let fee = client
    .get_fee_for_message(&message)
    .expect("Failed to get fee calculator");

        // let signature = client
        //     .send_and_confirm_transaction(&transaction)
        //     .expect("Failed to send transaction");
        //     println!(
        //     "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        //     signature
        // );

        // let transaction = Transaction::new_signed_with_payer(
        //     &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
        //     Some(&keypair.pubkey()),
        //     &vec![&keypair],
        //     recent_blockhash,
        // );

        // let signature = client
        //     .send_and_confirm_transaction(&transaction)
        //     .expect("Failed to send transaction");
        //     println!(
        //     "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        //     signature
        // );

        let sample_instruction = transfer(&keypair.pubkey(), &to_pubkey, balance);
        
        println!("Sample instruction: {:?}", sample_instruction);

    }

        #[test]
    fn base58_to_wallet() {
        println!("Input your private key as a base58 string:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file format is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }       

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a JSON byte array (e.g. [12,34,...]):");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("Your Base58-encoded private key is:");

        let base58 = bs58::encode(&wallet).into_string();
        println!("{:?}", base58);

        let public_key = Keypair::from_bytes(&wallet).unwrap().pubkey();
        println!("Public key: {}", public_key.to_string());
    }

    #[test]
    fn submit() {
        let rpc_client = RpcClient::new(RPC_URL);
        let mint = Keypair::new();
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Failed to read keypair file");
    
        let signer_pubkey = signer.pubkey();
        let seeds = &[b"prereqs", signer_pubkey.as_ref()];
        let (prereq_pda, _bump) = Pubkey::find_program_address(seeds,
        &Pubkey::from_str(TURBIN3_PREREQ_PROGRAM).unwrap());

        println!("Prereq PDA: {}", prereq_pda.to_string());

        let submit_rs_discriminator = vec![77, 124, 82, 163, 21, 133, 181, 206];

        let authority_bytes: &[u8; 10] = &[99, 111, 108, 108, 101, 99, 116, 105, 111, 110];
        let collection_pubkey = Pubkey::from_str(COLLECTION).unwrap();
        let authority_seeds = &[authority_bytes, collection_pubkey.as_ref()];
        let (authority_pda, _authority_bump) = Pubkey::find_program_address(authority_seeds,
        &Pubkey::from_str(TURBIN3_PREREQ_PROGRAM).unwrap());

        println!("Authority PDA: {}", authority_pda.to_string());

        let accounts = vec![
            AccountMeta::new(signer_pubkey, true), // user signer
            AccountMeta::new(prereq_pda, false), // PDA account
            AccountMeta::new(mint.pubkey(), true), // mint keypair
            AccountMeta::new(collection_pubkey, false), // collection
            AccountMeta::new_readonly(authority_pda, false), // authority (PDA)
            AccountMeta::new_readonly(Pubkey::from_str(MPL_CORE_PROGRAM).unwrap(), false), // mpl core program
            AccountMeta::new_readonly(Pubkey::from_str(SYSTEM_PROGRAM).unwrap(), false), // system program
        ];

        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let instruction = Instruction {
            program_id: Pubkey::from_str(TURBIN3_PREREQ_PROGRAM).unwrap(),
            accounts,
            data: submit_rs_discriminator,
        };

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer, &mint],
            blockhash,
        );
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
            println!(
            "Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );

    }
}
