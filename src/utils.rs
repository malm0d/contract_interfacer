use ethers::{
    providers::{Http, Provider},
    types::{Address, U256},
};

/// Create an instance of a provider
/// #Arguments
/// * `rpc_url` - RPC URL
/// 
/// #Returns
/// `Provider<Http>` - A new instance of `Provider<Http>`
pub async fn get_provider(rpc_url: &str) -> eyre::Result<Provider<Http>> {
    let provider = Provider::<Http>::try_from(rpc_url)?;
    Ok(provider)
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

/// Convenience function to convert `u32` to `U256`
/// #Arguments
/// * `amount` - Amount
/// 
/// #Returns
/// `U256` - An instance of `U256`
pub fn to_u256(amount: u32) -> U256 {
    U256::from(amount)
}