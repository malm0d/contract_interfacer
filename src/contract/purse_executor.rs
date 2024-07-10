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
/// A vector of length zero is: `Vec::<String>::new()`.
/// A vector that looks like this: `vec!["".to_string()]` will still have a length of 1.
/// 
/// ### Returns
/// `Result<()>` - Result indicating success or failure
pub fn validate_purse_calldata(func: &str, calldata: &Option<Vec<String>>) -> eyre::Result<()> {
    match (func, calldata) {
        // View functions do not have/require calldata so expect `None`
        ("address", None) | ("minted", None) | ("mintingCost", None) => Ok(()),

        // Single value calldata functions
        ("balanceOf", Some(data)) 
        | ("owned", Some(data))
        | ("mintERC721", Some(data))  if data.len() == 1 => Ok(()),

        // Two value calldata functions
        ("transfer", Some(data)) 
        | ("mint", Some(data)) if data.len() == 2 => Ok(()),

        // Unsupported functions
        (_, Some(_)) => Err(eyre::eyre!("Unsupported function: {}", func)),
        (_, None) => Err(eyre::eyre!("Unsupported function: {}", func)),
    }
}

/// Destruct the calldata based on the function of the Purse smart contract.
/// This is only relevant to contract functions that require calldata.
/// ### Arguments
/// * `func` - Function name
/// * `calldata` - Calldata
///
/// ### Returns
/// `(Option<String>, ... )` tuple containing up to seven `Option<String>` values
pub fn destruct_purse_calldata(
    func: &str,
    calldata: &Vec<String>
) -> (Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>) {
    match func {
        "balanceOf" | "owned" | "mintERC721" => {
            let val = calldata[0].clone();
            (Some(val), None, None, None, None, None, None)
        },
        "transfer" | "mint" => {
            let val1 = calldata[0].clone();
            let val2 = calldata[1].clone();
            (Some(val1), Some(val2), None, None, None, None, None)
        },
        _ => (None, None, None, None, None, None, None)
    }
}

/// Retrieves the recipient address and calldata value from the calldata for
/// single transfer and mint related functions only.
/// #### Note
/// For any ERC721 related mints, the recipient address returned by this function is `Address::zero()`.
/// Does not work for batch mint/transfer functions where calldata is a vector of values.
/// 
/// ### Arguments
/// * `func` - Function name
/// * `calldata` - Calldata
/// 
/// ### Returns
/// `(Address, U256)` - A tuple containing the recipient address and calldata value for
/// the single transfer or mint related function
pub fn transfer_or_mint_recipient_n_calldata(
    func: &str,
    calldata: &Vec<String>
) -> (Address, U256) {
    let (a, 
        b, 
        _c, 
        _d, 
        _e, 
        _f, 
        _g
    ) = destruct_purse_calldata(func, calldata);
    match func {
        "mintERC721" => {
            let recipient = Address::zero();
            let calldata_value = U256::from_dec_str(&a.unwrap()).unwrap();
            (recipient, calldata_value)
        },
        "transfer" | "mint" => {
            let recipient = Address::from_str(&a.unwrap()).unwrap();
            let calldata_value = U256::from_dec_str(&b.unwrap()).unwrap();
            (recipient, calldata_value)
        },
        _ => (Address::zero(), U256::from(0))
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
    /// * `message_value` - Message value
    /// * `calldata` - Calldata
    /// * `wallet` - Wallet
    /// 
    /// ### Returns
    /// `Result<Self>` - Result
    pub fn from_data(
        function: &str,
        message_value: &U256, 
        calldata: Vec<String>, 
        wallet: Wallet
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
                Ok(Purse404FunctionCall::Transfer(wallet, to, amount))
            },
            "mintERC721" => {
                let mint_unit = U256::from_dec_str(&calldata[0])?;
                Ok(Purse404FunctionCall::MintERC721(wallet, mint_unit, *message_value))
            },
            "mint" => {
                let to = Address::from_str(&calldata[0])?;
                let amount = U256::from_dec_str(&calldata[1])?;
                Ok(Purse404FunctionCall::Mint(wallet, to, amount))
            },
            _ => Err(eyre::eyre!("Unsupported function: {}", function)),
        }
    }
}

/// Purse404 Contract Functions Results (See: `purse_contract.rs`)
pub enum Purse404Results {
    Address(Address),
    U256Result(U256),
    U256VecResult(Vec<U256>),
    StringResult(String),
    StringVecResult(Vec<String>),
    StateChangeResult((String, String, String, String, String)),
}

pub struct Purse404Executor;

impl Purse404Executor {
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
    ) -> Result<Purse404Results> {
        match call {
            Purse404FunctionCall::Address => {
                let res = contract.address();
                Ok(Purse404Results::Address(res))
            },
            Purse404FunctionCall::BalanceOf(addr) => {
                let res = contract.balance_of(&addr).await?;
                Ok(Purse404Results::U256Result(res))
            },
            Purse404FunctionCall::Minted => {
                let res = contract.minted().await?;
                Ok(Purse404Results::U256Result(res))
            },
            Purse404FunctionCall::MintingCost => {
                let res = contract.minting_cost().await?;
                Ok(Purse404Results::U256Result(res))
            },
            Purse404FunctionCall::Owned(addr) => {
                let res = contract.owned(&addr).await?;
                Ok(Purse404Results::U256VecResult(res))
            },
            Purse404FunctionCall::Transfer(wallet, to, amount) => {
                let res = contract.transfer(
                    &wallet, 
                    &to, 
                    &amount
                ).await?;
                Ok(Purse404Results::StateChangeResult(res))
            },
            Purse404FunctionCall::MintERC721(wallet, mint_unit, msg_value) => {
                let res = contract.mint_erc721(
                    &wallet, 
                    &mint_unit, 
                    &msg_value
                ).await?;
                Ok(Purse404Results::StateChangeResult(res))
            },
            Purse404FunctionCall::Mint(wallet, to, amount) => {
                let res = contract.mint(
                    &wallet, 
                    &to, 
                    &amount
                ).await?;
                Ok(Purse404Results::StateChangeResult(res))
            }
        }
    }
}