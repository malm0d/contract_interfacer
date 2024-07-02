use crate::utils::str_wei_to_eth;
use core::panic;
use csv::{ WriterBuilder, ReaderBuilder };
use std::{ fs::{File, OpenOptions}, path::Path, error::Error};
use eyre::Result;
use serde::Deserialize;
use ethers::types::{Address, U256};

#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(rename = "Transaction Hash")]
    transaction_hash: String,
    #[serde(rename = "Derivation")]
    derivation: u32,
    #[serde(rename = "Sender")]
    sender: String,
    #[serde(rename = "Sender Balance Before (ETH)")]
    sender_balance_before_eth: f64,
    #[serde(rename = "Sender Balance After (ETH)")]
    sender_balance_after_eth: f64,
    #[serde(rename = "Sender Balance Before (ERC20)")]
    sender_balance_before_erc20: f64,
    #[serde(rename = "Sender Balance After (ERC20)")]
    sender_balance_after_erc20: f64,
    #[serde(rename = "Recipient")]
    recipient: String,
    #[serde(rename = "Recipient Balance Before (ETH)")]
    recipient_balance_before_eth: f64,
    #[serde(rename = "Recipient Balance After (ETH)")]
    recipient_balance_after_eth: f64,
    #[serde(rename = "Recipient Balance Before (ERC20)")]
    recipient_balance_before_erc20: f64,
    #[serde(rename = "Recipient Balance After (ERC20)")]
    recipient_balance_after_erc20: f64,
    #[serde(rename = "Function")]
    function: String,
    #[serde(rename = "Msg Value")]
    msg_value: f64,
    #[serde(rename = "Calldata Value")]
    calldata_value: f64,
    #[serde(rename = "Msg.sender Owned Token IDs")]
    msg_sender_owned_token_ids: String,
    #[serde(rename = "Tx Fee")]
    tx_fee: f64,
    #[serde(rename = "Gas Price")]
    gas_price: f64,
    #[serde(rename = "Gas Used")]
    gas_used: u64,
    #[serde(rename = "Receipt JSON")]
    receipt_json: String,
}

pub fn read_from_csv(file_path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let file = File::open(file_path).expect("Failed to open file");
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut records = Vec::new();
    for res in reader.deserialize() {
        let record: Record = res.expect("Failed to deserialize record");
        records.push(record);
    }

    Ok(records)
}

/// Logs the transaction information to a CSV file.
/// The CSV file is created if it does not exist.
/// The order of the columns is as follows: Transaction Hash, Derivation, Sender, Sender Balance Before (ETH),
/// Sender Balance After (ETH), Sender Balance Before (ERC20), Sender Balance After (ERC20), Recipient, 
/// Recipient Balance Before (ETH), Recipient Balance After (ETH), Recipient Balance Before (ERC20),
/// Recipient Balance After (ERC20), Function, Msg Value, Calldata Value, Msg.sender Owned Token IDs,
/// Tx Fee, Gas Price, Gas Used, Receipt JSON.
/// 
/// Additionally, if the file already exists, but the headers do not match the expected headers,
/// either in length, or content order, the program will panic.
/// 
/// ### Arguments
/// * `file_path` - File path
/// * `tx_hash` - Transaction hash
/// * `gas_price` - Gas price in gwei
/// * `gas_used` - Gas used in decimal
/// * `tx_fee` - Transaction fee in ETH
/// * `receipt_json_str` - Transaction receipt JSON
/// * `call_function` - Contract function called
/// * `derivation_number` - Derivation number of the address
/// * `msg_sender` - Message sender
/// * `sender_eth_bal_bef` - Sender balance, before (native)
/// * `sender_eth_bal_aft` - Sender balance, after (native)
/// * `sender_erc20_bal_bef` - Sender balance, before (ERC20)
/// * `sender_erc20_bal_aft` - Sender balance, after (ERC20)
/// * `msg_recipient` - Message recipient
/// * `recipient_eth_bal_bef` - Recipient balance, before (native)
/// * `recipient_eth_bal_aft` - Recipient balance, after (native)
/// * `recipient_erc20_bal_bef` - Recipient balance, before (ERC20)
/// * `recipient_erc20_bal_aft` - Recipient balance, after (ERC20)
/// * `msg_value` - Message value (optional)
/// * `calldata_value` - Calldata value (optional)
/// * `msg_sender_owned_token_ids` - Msg.sender Owned token IDs (optional)
/// 
/// ### Returns
/// `Result<(), Box<dyn std::error::Error>>` - Result
pub fn write_to_csv(
    file_path: &str,
    tx_hash: &str,
    gas_price: &str,
    gas_used: &str,
    tx_fee: &str,
    receipt_json_str: &str,
    call_function: &str,
    derivation_number: &u32,
    msg_sender: &Address,
    sender_eth_bal_bef: Option<U256>,
    sender_eth_bal_aft: Option<U256>,
    sender_erc20_bal_bef: Option<U256>,
    sender_erc20_bal_aft: Option<U256>,
    msg_recipient: &Address,
    recipient_eth_bal_bef: Option<U256>,
    recipient_eth_bal_aft: Option<U256>,
    recipient_erc20_bal_bef: Option<U256>,
    recipient_erc20_bal_aft: Option<U256>,
    msg_value: Option<U256>,
    calldata_value: Option<U256>,
    msg_sender_owned_token_ids: Option<Vec<U256>>,
) -> Result<()> {
    let path = Path::new(file_path);
    let file_exists = match path.try_exists() {
        Ok(exists) => exists,
        Err(_) => return Err(eyre::eyre!("File existence cannot be confirmed, check dir permissions"))
    };
    let headers = [
        "Transaction Hash", "Derivation", 
        "Sender", 
        "Sender Balance Before (ETH)", "Sender Balance After (ETH)", 
        "Sender Balance Before (ERC20)", "Sender Balance After (ERC20)", 
        "Recipient", 
        "Recipient Balance Before (ETH)", "Recipient Balance After (ETH)",
        "Recipient Balance Before (ERC20)", "Recipient Balance After (ERC20)",
        "Function", "Msg Value", "Calldata Value", "Msg.sender Owned Token IDs", 
        "Tx Fee", "Gas Price", "Gas Used", "Receipt JSON"
    ];

    if file_exists {
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_path(path)
            .expect("Failed to read file");

        let headers_read = reader.headers().expect("Failed to read headers");
        if (headers_read.len() != headers.len()) 
        || (!headers_read.iter().zip(headers.iter()).all(|(a, b)| a == *b)) {
            panic!("Headers length or content order mismatch");
        }
    }

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(file_exists)
        .open(path)
        .expect("Failed to create/open file");

    let mut writer = WriterBuilder::new()
        .has_headers(!file_exists)
        .from_writer(file);

    if !file_exists {
        writer.write_record(&headers).expect("Failed to write headers");
    }

    let default_u256 = U256::from(0);

    let sender_eth_bal_bef = sender_eth_bal_bef.unwrap_or_else(|| default_u256);
    let sender_eth_bal_aft = sender_eth_bal_aft.unwrap_or_else(|| default_u256);
    let sender_erc20_bal_bef = sender_erc20_bal_bef.unwrap_or_else(|| default_u256);
    let sender_erc20_bal_aft = sender_erc20_bal_aft.unwrap_or_else(|| default_u256);

    let recipient_eth_bal_bef = recipient_eth_bal_bef.unwrap_or_else(|| default_u256);
    let recipient_eth_bal_aft = recipient_eth_bal_aft.unwrap_or_else(|| default_u256);
    let recipient_erc20_bal_bef = recipient_erc20_bal_bef.unwrap_or_else(|| default_u256);
    let recipient_erc20_bal_aft = recipient_erc20_bal_aft.unwrap_or_else(|| default_u256);

    let msg_value = msg_value.unwrap_or_else(|| default_u256);
    let calldata_value = calldata_value.unwrap_or_else(|| default_u256);

    let msg_sender_owned_token_ids = msg_sender_owned_token_ids
        .map(|vec| vec.into_iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join(","))
        .unwrap_or_default();

    writer.write_record(&[
        tx_hash,
        derivation_number.to_string().as_str(),
        msg_sender.to_string().as_str(),
        str_wei_to_eth(&sender_eth_bal_bef.to_string()).as_str(),
        str_wei_to_eth(&sender_eth_bal_aft.to_string()).as_str(),
        str_wei_to_eth(&sender_erc20_bal_bef.to_string()).as_str(),
        str_wei_to_eth(&sender_erc20_bal_aft.to_string()).as_str(),
        msg_recipient.to_string().as_str(),
        str_wei_to_eth(&recipient_eth_bal_bef.to_string()).as_str(),
        str_wei_to_eth(&recipient_eth_bal_aft.to_string()).as_str(),
        str_wei_to_eth(&recipient_erc20_bal_bef.to_string()).as_str(),
        str_wei_to_eth(&recipient_erc20_bal_aft.to_string()).as_str(),
        call_function,
        str_wei_to_eth(&msg_value.to_string()).as_str(),
        str_wei_to_eth(&calldata_value.to_string()).as_str(),
        msg_sender_owned_token_ids.as_str(),
        tx_fee,
        gas_price,
        gas_used,
        receipt_json_str
    ]).expect("Failed to write record");

    writer.flush().expect("Failed to flush writer");
    println!("Transaction hash: {}, from address: {}, added to file: {}", tx_hash, msg_sender, file_path);

    Ok(())
}