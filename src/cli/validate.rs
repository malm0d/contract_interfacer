/// Validate the calldata based on the function of the Purse smart contract.
/// ### Arguments
/// * `func` - Function name
/// * `calldata` - Calldata
/// 
/// #### Note
/// An vector of length zero is: `Vec::<String>::new()`.
/// A vector that looks like this: `vec!["".to_string()]` will still have a length of 1.
/// 
/// ### Returns
/// `Result<()>` - Result indicating success or failure
pub fn validate_purse_calldata(func: &str, calldata: &Option<Vec<String>>) -> eyre::Result<()> {
    match (func, calldata) {
        // View functions do not have/require calldata so expect `None`
        ("address", None) | ("minted", None) | ("mintingCost", None) => Ok(()),

        // Write functions
        ("balanceOf", Some(data)) 
        | ("owned", Some(data)) if data.len() == 1 => Ok(()),

        ("transfer", Some(data)) 
        | ("mintERC721", Some(data)) 
        | ("mint", Some(data)) if data.len() == 2 => Ok(()),

        // Unsupported functions
        (_, Some(_)) => eyre::bail!("Unsupported function"),
        (_, None) => eyre::bail!("Unsupported function"),
    }
}