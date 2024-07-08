use eyre::Result;
use std::str::FromStr;
use ethers::{
    types::{Address, U256},
    providers::Middleware,
};
use crate::{
    contract::purse_contract::Purse404Contract,
    wallet::Wallet,
};

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
        | ("owned", Some(data))
        | ("mintERC721", Some(data))  if data.len() == 1 => Ok(()),

        ("transfer", Some(data)) 
        | ("mint", Some(data)) if data.len() == 2 => Ok(()),

        // Unsupported functions
        (_, Some(_)) => eyre::bail!("Unsupported function"),
        (_, None) => eyre::bail!("Unsupported function"),
    }
}

/// Purse404 Contract Functions (See: `purse_contract.rs`)
pub enum Purse404FunctionCall {
    Address,
    BalanceOf(Address),
    Minted,
    MintingCost,
    Owned(Address),
    Transfer(Wallet, Address, U256),
    MintERC721(Wallet, U256, U256),
    Mint(Wallet, Address, U256),
}

impl Purse404FunctionCall {
    /// Create a new instance of `Purse404FunctionCall` from the given function name, message value, 
    /// calldata, and wallet.
    /// ### Arguments
    /// * `function` - Function name
    /// * `msg_value` - Message value
    /// * `calldata` - Calldata
    /// * `wallet` - Wallet
    /// 
    /// ### Returns
    /// `Result<Self>` - Result
    pub fn from_data(
        function: &str,
        msg_value: &U256, 
        calldata: &Vec<String>, 
        wallet: &Wallet
    ) -> Result<Self, eyre::Report> {
        match function {
            "address" => Ok(Purse404FunctionCall::Address),
            "balanceOf" => {
                let addr = Address::from_str(&calldata[0])?;
                Ok(Purse404FunctionCall::BalanceOf(addr))
            }
            "minted" => Ok(Purse404FunctionCall::Minted),
            "mintingCost" => Ok(Purse404FunctionCall::MintingCost),
            "owned" => {
                let addr = Address::from_str(&calldata[0])?;
                Ok(Purse404FunctionCall::Owned(addr))
            },
            "transfer" => {
                let to = Address::from_str(&calldata[0])?;
                let amount = U256::from_dec_str(&calldata[1])?;
                Ok(Purse404FunctionCall::Transfer(wallet.clone(), to, amount))
            },
            "mintERC721" => {
                let mint_unit = U256::from_dec_str(&calldata[0])?;
                Ok(Purse404FunctionCall::MintERC721(wallet.clone(), mint_unit, *msg_value))
            },
            "mint" => {
                let to = Address::from_str(&calldata[0])?;
                let amount = U256::from_dec_str(&calldata[1])?;
                Ok(Purse404FunctionCall::Mint(wallet.clone(), to, amount))
            },
            _ => Err(eyre::eyre!("Unsupported function")),
        }
    }
}

/// Purse404 Contract Functions Results (See: `purse_contract.rs`)
pub enum Purse404Results {
    Address(Address),
    U256Result(Result<U256, eyre::Report>),
    U256VecResult(Result<Vec<U256>, eyre::Report>),
    StringResult(Result<String, eyre::Report>),
    StringVecResult(Result<Vec<String>, eyre::Report>),
    StateChangeResult(Result<(String, String, String, String, String), eyre::Report>),
}

pub struct PurseExecutor;

impl PurseExecutor {
    /// Execute the given function call on the Purse404 contract.
    /// ### Arguments
    /// * `contract` - Purse404 contract
    /// * `call` - Function call
    /// 
    /// ### Returns
    /// `Purse404Results` - Results
    pub async fn execute_fn<M: Middleware + 'static>(
        contract: &Purse404Contract<M>,
        call: Purse404FunctionCall,
    ) -> Purse404Results {
        match call {
            Purse404FunctionCall::Address => {
                let res = contract.address();
                Purse404Results::Address(res)
            },
            Purse404FunctionCall::BalanceOf(addr) => {
                let res = contract.balance_of(&addr).await;
                Purse404Results::U256Result(res)
            },
            Purse404FunctionCall::Minted => {
                let res = contract.minted().await;
                Purse404Results::U256Result(res)
            },
            Purse404FunctionCall::MintingCost => {
                let res = contract.minting_cost().await;
                Purse404Results::U256Result(res)
            },
            Purse404FunctionCall::Owned(addr) => {
                let res = contract.owned(&addr).await;
                Purse404Results::U256VecResult(res)
            },
            Purse404FunctionCall::Transfer(wallet, to, amount) => {
                let res = contract.transfer(
                    &wallet, 
                    &to, 
                    &amount
                ).await;
                Purse404Results::StateChangeResult(res)
            },
            Purse404FunctionCall::MintERC721(wallet, mint_unit, msg_value) => {
                let res = contract.mint_erc721(
                    &wallet, 
                    &mint_unit, 
                    &msg_value
                ).await;
                Purse404Results::StateChangeResult(res)
            },
            Purse404FunctionCall::Mint(wallet, to, amount) => {
                let res = contract.mint(
                    &wallet, 
                    &to, 
                    &amount
                ).await;
                Purse404Results::StateChangeResult(res)
            }
        }
    }
}