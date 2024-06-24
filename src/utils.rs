use ethers::{
    providers::{Http, Provider},
    types::Address,
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