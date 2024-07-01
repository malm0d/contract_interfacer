use core::panic;
use csv::{ WriterBuilder, ReaderBuilder };
use std::{ fs::OpenOptions, path::Path, str::FromStr };
use eyre::Result;
use serde_json::Value;
use ethers::{
    providers::{Http, Provider},
    types::{Address, U256},
    middleware::Middleware
};
use bigdecimal::{BigDecimal, FromPrimitive};

/// Create an instance of a provider
/// #Arguments
/// * `rpc_url` - RPC URL
/// 
/// #Returns
/// `Provider<Http>` - A new instance of `Provider<Http>`
pub async fn get_provider(rpc_url: &str) -> eyre::Result<Provider<Http>> {
    let provider = Provider::<Http>::try_from(rpc_url);
    match provider {
        Ok(prov) => Ok(prov),
        Err(e) => Err(eyre::eyre!("Failed to get provider: {}", e))
    }
}

/// Get the balance (native) of the given address.
/// This is NOT the ERC20 balance.
/// #Arguments
/// * `prov` - Provider
/// * `address` - Address
/// 
/// #Returns
/// `Result<U256>` - Result
pub async fn get_native_balance(prov: &Provider<Http>, address: &Address) -> Result<U256> {
    let balance = prov.clone().get_balance(*address, None).await;
    match balance {
        Ok(bal) => Ok(bal),
        Err(e) => Err(eyre::eyre!("Failed to get balance: {}", e))
    }
}

/// Converts the given string slice to an `Address` (H160) type
/// #Arguments
/// * `str_slice` - String slice
/// 
/// #Returns
/// `Address` - An instance of `Address`
pub fn to_address_type(str_slice: &str) -> Address {
    str_slice.parse::<Address>().unwrap()
}

/// Convenience function to convert `u128` to `U256`
/// #Arguments
/// * `amount` - Amount
/// 
/// #Returns
/// `U256` - An instance of `U256`
pub fn to_u256(amount: u128) -> U256 {
    U256::from(amount)
}

pub fn str_wei_to_eth(wei: &str) -> String {
    let wei_bd = BigDecimal::from_str(wei).expect("Invalid input str");
    let eth_per_wei_bd = BigDecimal::from_i64(1_000_000_000_000_000_000).unwrap();
    let eth_bd = wei_bd / eth_per_wei_bd;
    eth_bd.to_string()
}

/// Extracts the transaction hash from the transaction receipt JSON
/// #Arguments
/// * `receipt_json` - Transaction receipt JSON
/// 
/// #Returns
/// `String` - Transaction hash
pub fn get_tx_hash(receipt_json: &str) -> String {
    let receipt: Value = serde_json::from_str(
        &receipt_json
    ).expect("Failed to parse receipt JSON");
    if let Some(tx_hash) = receipt["transactionHash"].as_str() {
        return tx_hash.to_string();
    } else {
        panic!("Failed to get transaction hash from receipt: Not found");
    }
}

/// Extracts the gas used from the transaction receipt JSON
/// #Arguments
/// * `receipt_json` - Transaction receipt JSON
/// 
/// #Returns
/// `String` - Gas used in decimal
pub fn get_gas_used(receipt_json: &str) -> String {
    let receipt: Value = serde_json::from_str(
        &receipt_json
    ).expect("Failed to parse receipt JSON");
    if let Some(gas_used) = receipt["gasUsed"].as_str() {
        let hexa = gas_used.trim_start_matches("0x");
        let gas_used_val = i64::from_str_radix(hexa, 16).unwrap();
        gas_used_val.to_string()
    } else {
        panic!("Failed to get gas used from receipt: Not found");
    }
}

/// Extracts the gas price from the transaction receipt JSON
/// #Arguments
/// * `receipt_json` - Transaction receipt JSON
/// 
/// #Returns
/// `String` - Gas price in gwei
pub fn get_gas_price(receipt_json: &str) -> String {
    let receipt: Value = serde_json::from_str(
        &receipt_json
    ).expect("Failed to parse receipt JSON");
    if let Some(gas_price) = receipt["effectiveGasPrice"].as_str() {
        let hexa = gas_price.trim_start_matches("0x");
        let gas_px_wei = i64::from_str_radix(hexa, 16).unwrap();
        let gas_px_gwei = gas_px_wei as f64 / 1_000_000_000.0;
        gas_px_gwei.to_string()
    } else {
        panic!("Failed to get gas price from receipt: Not found");
    }
}

/// Calculates the transaction fee in ETH.
/// The transaction fee can be calculated by multiplying the gas used by the gas price.
/// #Arguments
/// * `receipt_json` - Transaction receipt JSON
/// 
/// #Returns
/// `String` - Transaction fee in ETH
pub fn calc_tx_fee(receipt_json: &str) -> String {
    let receipt: Value = serde_json::from_str(
        &receipt_json
    ).expect("Failed to parse receipt JSON");

    let gas_used = match receipt["gasUsed"].as_str() {
        Some(gu) => gu.trim_start_matches("0x"),
        None => panic!("Failed to get gas used from receipt: Not found")
    };

    let gas_price = match receipt["effectiveGasPrice"].as_str() {
        Some(gp) => gp.trim_start_matches("0x"),
        None => panic!("Failed to get gas price from receipt: Not found")
    };

    let gas_used_val = i64::from_str_radix(gas_used, 16).unwrap() as f64;
    let gas_price_wei = i64::from_str_radix(gas_price, 16).unwrap() as f64;
    let tx_fee_eth = gas_used_val * gas_price_wei / 1_000_000_000_000_000_000.0;

    tx_fee_eth.to_string()
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
/// #Arguments
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
/// #Returns
/// `Result<(), Box<dyn std::error::Error>>` - Result
pub fn write_to_csv(
    file_path: &str,
    tx_hash: &str,
    gas_price: &str,
    gas_used: &str,
    tx_fee: &str,
    receipt_json_str: &str,
    call_function: &str,
    derivation_number: &u64,
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