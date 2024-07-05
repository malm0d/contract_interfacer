use dotenv::dotenv;
use std::sync::Arc;
use clap::Parser;
use eyre::Result;
use ethers::types::U256;
use super::{ args::CliArgs, validate::* };
use crate::{
    file::{
        Record, 
        read_from_csv, 
        write_to_csv,
    },
    utils::{
        to_address_type, 
        to_u256, 
        get_provider, 
        get_native_balance,
    },
    wallet::Wallet,
    contract::PurseToken404Contract,
    constants::PURSE_ETH_ADDRESS,
};

#[derive(Debug, Parser)]
pub struct CliCommand {
    /// All Cli args
    #[clap(flatten)]
    cli_args: CliArgs,
}

impl CliCommand {
    /// Execute the command
    pub async fn execute(self) -> eyre::Result<()> {
        dotenv().ok();
        let chain_id = self.cli_args.chain_id;
        let phrase = std::env::var("MNEMONIC").unwrap();
        let mut derivation_num_arg = self.cli_args.derivation_number;
        let file_path = self.cli_args.file_path;

        match read_from_csv(&file_path) {
            Ok(records) => {
                let mut derivation_numbers: Vec<u32> = records
                    .iter()
                    .map(|record| record.derivation)
                    .collect();

                if derivation_numbers.len() == 0 {
                    eprintln!("An existing file must have at least one record with a derivation number.");
                    Err(eyre::eyre!("No recorded derivation numbers found in: {}. Halting..", file_path))?;
                }

                derivation_numbers.sort();
                let highest = *derivation_numbers.last().unwrap();
                println!("Recorded derivation numbers: {:?} \n", derivation_numbers);
                println!("Highest derivation number last used: {:?} \n", highest);

                // If `derivation_number_arg` is 0 (default), use the next highest number
                if derivation_num_arg == 0 {
                    derivation_num_arg = highest + 1;
                    println!("Using next derivation number: {} \n", derivation_num_arg);
                } else {
                    println!("Using provided derivation number: {} \n", derivation_num_arg);
                }
            },
            Err(_e) => {
                println!("Starting new file: {}", file_path);
                println!("Defaulting derivation number to 0 for the current execution context \n");
            }
        }

        let prov = match chain_id {
            1 => get_provider(
                std::env::var("MAINNET_RPC").unwrap().as_str()
            ).await.unwrap(),
            11155111 => get_provider(
                std::env::var("SEPOLIA_RPC").unwrap().as_str()
            ).await.unwrap(),
            _ => {
                Err(eyre::eyre!("Unsupported chain id: {}. Halting..", chain_id))?
            }
        };

        let wallet = Wallet::from_phrase(
            phrase.as_str(),
            derivation_num_arg,
            11155111
        ).unwrap();
        let msg_sender_address = wallet.address();
        let msg_value = self.cli_args.msg_value;

        let call_function = self.cli_args.function;
        let calldata = self.cli_args.calldata;
        let _ = validate_purse_calldata(&call_function, &calldata);
        let calldata_vec = calldata.unwrap_or_default();

        let purse_token = PurseToken404Contract::new(
            to_address_type(PURSE_ETH_ADDRESS),
            &Arc::new(prov.clone()),
        );

        Ok(())
    }
}