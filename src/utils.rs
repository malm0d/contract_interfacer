use core::panic;
use std::str::FromStr;
use eyre::Result;
use serde_json::Value;
use ethers::{
    providers::{Http, Provider},
    types::{Address, U256},
    middleware::Middleware
};
use bigdecimal::{BigDecimal, FromPrimitive};

/// Create an instance of a provider
/// ### Arguments
/// * `rpc_url` - RPC URL
/// 
/// ### Returns
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
/// ### Arguments
/// * `prov` - Provider
/// * `address` - Address
/// 
/// ### Returns
/// `Result<U256>` - Result
pub async fn get_native_balance(prov: &Provider<Http>, address: &Address) -> Result<U256> {
    let balance = prov.clone().get_balance(*address, None).await;
    match balance {
        Ok(bal) => Ok(bal),
        Err(e) => Err(eyre::eyre!("Failed to get balance: {}", e))
    }
}

/// Converts the given string slice to an `Address` (H160) type
/// ### Arguments
/// * `str_slice` - String slice
/// 
/// ### Returns
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

/// Parses U256 from the given `&str`
/// ### Arguments
/// * `s` - String slice of the amount to parse
/// 
/// ### Returns
/// `Result<U256, String>` - Result
pub fn parse_u256(s: &str) -> Result<U256, String> {
    U256::from_str_radix(s, 10).map_err(|_| format!("String {s} is not a valid U256"))
}

/// Converts the given string slice of a WEI value to an ETH value
/// ### Arguments
/// * `wei` - WEI value as a string slice
/// 
/// ### Returns
/// `String` - ETH value as a string
pub fn str_wei_to_eth(wei: &str) -> String {
    let wei_bd = BigDecimal::from_str(wei).expect("Invalid input str");
    let eth_per_wei_bd = BigDecimal::from_i64(1_000_000_000_000_000_000).unwrap();
    let eth_bd = wei_bd / eth_per_wei_bd;
    eth_bd.to_string()
}

/// Extracts the transaction hash from the transaction receipt JSON
/// ### Arguments
/// * `receipt_json` - Transaction receipt JSON
/// 
/// ### Returns
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
/// ### Arguments
/// * `receipt_json` - Transaction receipt JSON
/// 
/// ### Returns
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
/// ### Arguments
/// * `receipt_json` - Transaction receipt JSON
/// 
/// ### Returns
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
/// ### Arguments
/// * `receipt_json` - Transaction receipt JSON
/// 
/// ### Returns
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