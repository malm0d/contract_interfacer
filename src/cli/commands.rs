use dotenv::dotenv;
use std::sync::Arc;
use clap::Parser;
use super::args::ContractCliArgs;
use crate::{
    file::{
        read_from_csv, 
        write_to_csv,
    },
    utils::{
        to_address_type, 
        get_provider, 
        get_native_balance,
    },
    wallet::Wallet,
    contract::{
        purse_contract::Purse404Contract,
        purse_executor::{
            validate_purse_calldata,
            transfer_or_mint_recipient_n_calldata,
            Purse404FunctionCall,
            Purse404Executor,
            Purse404Results,
        },
    },
    constants::PURSE_ETH_ADDRESS,
};

#[derive(Debug, Parser)]
pub struct PurseCommand {
    /// All Cli args
    #[clap(flatten)]
    cli_args: ContractCliArgs,
}

impl PurseCommand {
    /// Execute the command
    pub async fn execute(self) -> eyre::Result<()> {
        println!("> Executing Purse command \n");

        dotenv().ok();
        let cid = self.cli_args.chain_id;
        let phrase = std::env::var("MNEMONIC")?;
        let mut derivation_num_arg = self.cli_args.derivation_number;
        let file_path = self.cli_args.file_path;

        match read_from_csv(&file_path) {
            Ok(records) => {
                let mut derivation_numbers: Vec<u32> = records
                    .iter()
                    .map(|record| record.derivation)
                    .collect();

                if derivation_numbers.len() == 0 {
                    eprintln!("> An existing file must have at least one record with a derivation number.");
                    return Err(eyre::eyre!("> No recorded derivation numbers found in: {}. Halting...", file_path))
                }

                derivation_numbers.sort();
                let highest = *derivation_numbers.last().unwrap();
                println!("> Recorded derivation numbers: {:?} \n", derivation_numbers);
                println!("> Highest derivation number last used: {:?} \n", highest);

                // If `derivation_number_arg` is 0 (default), use the next highest number
                if derivation_num_arg == 0 {
                    derivation_num_arg = highest + 1;
                    println!("> Using next derivation number: {} \n", derivation_num_arg);
                } else {
                    println!("> Using provided derivation number: {} \n", derivation_num_arg);
                }
            },
            Err(_e) => {
                println!("> Starting new file: \"{}\" ", file_path);
                println!("> File will only be created if a write transaction is executed and completed successfully \n");
                println!("> Defaulting derivation number to 0 for the current execution context \n");
            }
        }

        let prov = match cid {
            1 => get_provider(
                std::env::var("MAINNET_RPC").unwrap().as_str()
            ).await?,
            11155111 => get_provider(
                std::env::var("SEPOLIA_RPC").unwrap().as_str()
            ).await?,
            _ => {
                return Err(eyre::eyre!("Unsupported chain id: {}. Halting...", cid))
            }
        };

        let wallet = Wallet::from_phrase(
            phrase.as_str(),
            derivation_num_arg,
            cid
        ).unwrap();
        let msg_sender_address = wallet.address();

        let msg_value = self.cli_args.msg_value;
        let call_fn = self.cli_args.function;
        let calldata = self.cli_args.calldata;
        validate_purse_calldata(&call_fn, &calldata)?;
        let cdata_vec = calldata.unwrap_or_default();

        let (msg_recipient_address, calldata_value) = transfer_or_mint_recipient_n_calldata(
            &call_fn, 
            &cdata_vec
        );

        let purse_token = Purse404Contract::new(
            to_address_type(PURSE_ETH_ADDRESS),
            &Arc::new(prov.clone()),
        );
        
        let function_call = Purse404FunctionCall::from_data(
            &call_fn, 
            &msg_value, 
            cdata_vec.clone(), 
            wallet
        )?;

        let sender_eth_bal_bef = get_native_balance(&prov, &msg_sender_address).await?;
        let sender_erc20_bal_bef = purse_token.balance_of(&msg_sender_address).await?;
        let recipient_eth_bal_bef = get_native_balance(&prov, &msg_recipient_address).await?;
        let recipient_erc20_bal_bef = purse_token.balance_of(&msg_recipient_address).await?;

        let tx_result = Purse404Executor::execute_fn(
            &purse_token, 
            function_call
        ).await?;
        
        match tx_result {
            Purse404Results::Address(addr) => {
                println!("> Purse404 contract address: {:?}", addr);
            },
            Purse404Results::U256Result(res) => {
                println!("> Function call: {} \n Calldata: {}", call_fn, cdata_vec.join(", "));
                println!("> Result: {}", res);
            },
            Purse404Results::U256VecResult(res) => {
                println!("> Function call: {} \n Calldata: {}", call_fn, cdata_vec.join(", "));
                println!("> Result: {:?}", res);
            },
            Purse404Results::StringResult(res) => {
                println!("> Function call: {} \n Calldata: {}", call_fn, cdata_vec.join(", "));
                println!("> Result: {}", res);
            },
            Purse404Results::StringVecResult(res) => {
                println!("> Function call: {} \n Calldata: {}", call_fn, cdata_vec.join(", "));
                println!("> Result: {:?}", res);
            },
            Purse404Results::StateChangeResult((
                tx_hash,
                gas_price,
                gas_used,
                tx_fees,
                tx_receipt_json
            )) => {
                let msg_sender_owned_token_ids = purse_token.owned(&msg_sender_address).await.unwrap();
                let sender_eth_bal_aft = get_native_balance(&prov, &msg_sender_address).await.unwrap();
                let sender_erc20_bal_aft = purse_token.balance_of(&msg_sender_address).await.unwrap();
                let recipient_eth_bal_aft = get_native_balance(&prov, &msg_recipient_address).await.unwrap();
                let recipient_erc20_bal_aft = purse_token.balance_of(&msg_recipient_address).await.unwrap();
            
                let _ = write_to_csv(
                    &file_path,
                    &tx_hash,
                    &gas_price,
                    &gas_used,
                    &tx_fees,
                    &tx_receipt_json,
                    &call_fn,
                    derivation_num_arg,
                    msg_sender_address,
                    Some(sender_eth_bal_bef),
                    Some(sender_eth_bal_aft),
                    Some(sender_erc20_bal_bef),
                    Some(sender_erc20_bal_aft),
                    msg_recipient_address,
                    Some(recipient_eth_bal_bef),
                    Some(recipient_eth_bal_aft),
                    Some(recipient_erc20_bal_bef),
                    Some(recipient_erc20_bal_aft),
                    Some(msg_value),
                    Some(calldata_value),
                    Some(msg_sender_owned_token_ids)
                );
            }
        }

        Ok(())
    }
}