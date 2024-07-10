use eyre::Result;
use std::sync::Arc;
use ethers::{
    prelude::SignerMiddleware, 
    providers::Middleware, 
    types::{ Address, U256 },
    contract::abigen
};
use crate::utils::{
    get_tx_hash, 
    get_gas_price, 
    get_gas_used,
    calc_tx_fee,
};
use crate::wallet::Wallet;

abigen!(
    Purse404,
    "abi/purseTokenAbi.json",
);

/// Wrapper around Purse404 contract
/// With traits `Clone` and `Debug`
/// Fields:
/// * `address` - Address in `Address` type
/// * `contract` - Purse404 contract instance
/// * `provider` - Provider
#[derive(Clone, Debug)]
pub struct Purse404Contract<M: Middleware + 'static> {
    address: Address,
    contract: Purse404<M>,
    provider: Arc<M>,
}

impl<M: Middleware + 'static> Purse404Contract<M> {
    /// Create a new `Purse404Contract` instance
    /// ### Arguments
    /// * `address` - Address of the deployed contract
    /// * `provider` - Network Provider
    /// 
    /// ### Returns
    /// `Self` - A new `Purse404Contract` instance
    pub fn new(address: Address, provider: &Arc<M>) -> Self {
        let contract = Purse404::new(
            address, 
            Arc::clone(provider)
        );
        Self { address, contract, provider: Arc::clone(provider) }
    }

    /// Returns the address of the contract: `Address`
    /// ### Returns
    /// `Address` - The address of the contract
    pub fn address(&self) -> Address {
        self.address
    }

    /// Returns an instance of the provider: `Arc<M>`
    /// ### Returns
    /// `Arc<M>` - An instance of the provider
    pub fn provider(&self) -> Arc<M> {
        self.provider.clone()
    }

    /// Gets the balance of the given address
    /// ### Arguments
    /// * `addr` - Address
    /// 
    /// ### Returns
    /// `Result<U256>` - A `U256` type
    pub async fn balance_of(&self, addr: &Address) -> Result<U256> {
        let res = self.contract.balance_of(*addr).call().await;
        match res {
            Ok(balance) => Ok(balance),
            Err(e) => Err(eyre::Report::from(e))
        }
    }

    /// Gets the current minted NFT amount
    /// 
    /// ### Returns
    /// `Result<U256>` - A `U256` type
    pub async fn minted(&self) -> Result<U256> {
        let res = self.contract.minted().call().await;
        match res {
            Ok(minted) => Ok(minted),
            Err(e) => Err(eyre::Report::from(e))
        }
    }

    /// Gets the current minting cost to mint an NFT
    /// 
    /// ### Returns
    /// `Result<U256>` - A `U256` type
    pub async fn minting_cost(&self) -> Result<U256> {
        let res = self.contract.minting_cost().call().await;
        match res {
            Ok(cost) => Ok(cost),
            Err(e) => Err(eyre::Report::from(e))
        }
    }

    /// Retrieves all NFT token IDs owned by the given address
    /// ### Arguments
    /// * `owner` - an `Address` reference
    /// 
    /// ### Returns
    /// `Result<Vec<U256>>` - A vector of `U256` types
    pub async fn owned(&self, owner: &Address) -> Result<Vec<U256>> {
        let res = self.contract.owned(*owner).call().await;
        match res {
            Ok(owned) => Ok(owned),
            Err(e) => Err(eyre::Report::from(e))
        }
    }

    /// Transfer the given amount (ERC20), from a `Wallet` to the given address.
    /// ### Arguments
    /// * `from` - a `Wallet` reference, the sender of the transfer
    /// * `to` - an `Address` reference, the recipient of the transfer
    /// * `amount` - a `U256` reference, the amount to transfer
    /// 
    /// ### Returns
    /// `Result<(String, String, String, String, String)>` - A tuple of transaction hash, 
    /// gas price, gas used, transaction fees, and transaction receipt JSON
    pub async fn transfer(
        &self, 
        from: &Wallet, 
        to_address: &Address, 
        amount: &U256
    ) -> Result<(String, String, String, String, String)> {
        let signer_middleware = SignerMiddleware::new(
            self.provider.clone(),
            from.signer.clone()
        );
        let contract_with_signer = Purse404::new(
            self.address.clone(),
            Arc::new(signer_middleware)
        );

        let tx = contract_with_signer.transfer(*to_address, *amount);
        let pending_tx = match tx.send().await {
            Ok(pending_tx) => {
                println!(
                    "Transaction sent, from: {:?}, to: {:?}, amount (wei): {} \n", 
                    from.address(), 
                    to_address, 
                    amount
                );
                println!("Waiting...");
                pending_tx
            },
            Err(e) => {
                return Err(eyre::eyre!("Failed to send transaction: {}", e))
            }
        };
        let receipt = match pending_tx.await {
            Ok(receipt) => receipt,
            Err(e) => {
                return Err(eyre::eyre!("Unexpected error occurred: {}", e))
            }
        };

        let json_str = serde_json::to_string(&receipt)?;
        let tx_hash = get_tx_hash(&json_str);
        let gas_price = get_gas_price(&json_str);
        let gas_used = get_gas_used(&json_str);
        let tx_fee = calc_tx_fee(&json_str);
        
        println!("Transaction hash: {}", tx_hash);
        println!("Gas price (gwei): {}", gas_price);
        println!("Gas used: {}", gas_used);
        println!("Transaction fee (ETH): {}", tx_fee);
        println!("Transfer transaction receipt: {} \n", json_str);

        Ok((tx_hash, gas_price, gas_used, tx_fee, json_str))
    }

    /// Mint ERC721 token(s) to the given wallet.
    /// ### Arguments
    /// * `mint_to` - a `Wallet` reference, the sender of the transaction
    /// * `mint_unit` - a `U256` reference, the amount to mint (treated as integer)
    /// * `message_value` - a `U256` reference, the msg value to send with the transaction
    /// 
    /// ### Returns
    /// `Result<(String, String, String, String, String)>` - A tuple of transaction hash, 
    /// gas price, gas used, transaction fees, and transaction receipt JSON
    pub async fn mint_erc721(
        &self,
        mint_to: &Wallet, 
        mint_units: &U256,
        message_value: &U256 
    ) -> Result<(String, String, String, String, String)> {
        let signer_middleware = SignerMiddleware::new(
            self.provider.clone(), 
            mint_to.signer.clone()
        );
        let contract_with_signer = Purse404::new(
            self.address.clone(), 
            Arc::new(signer_middleware)
        );

        let tx = contract_with_signer.mint_erc721(*mint_units).value(*message_value);
        let pending_tx = match tx.send().await {
            Ok(pending_tx) => {
                println!(
                    "Transaction sent, from: {}, to: {}, amount (nfts): {} \n", 
                    mint_to.address(), 
                    self.address(), 
                    mint_units
                );
                println!("Waiting...");
                pending_tx
            },
            Err(e) => {
                return Err(eyre::eyre!("Failed to send transaction: {}", e))
            }
        };
        let receipt = match pending_tx.await {
            Ok(receipt) => receipt,
            Err(e) => {
                return Err(eyre::eyre!("Unexpected error occurred: {}", e))
            }
        };

        let json_str = serde_json::to_string(&receipt)?;
        let tx_hash = get_tx_hash(&json_str);
        let gas_price = get_gas_price(&json_str);
        let gas_used = get_gas_used(&json_str);
        let tx_fee = calc_tx_fee(&json_str);

        println!("Transaction hash: {}", tx_hash);
        println!("Gas price (gwei): {}", gas_price);
        println!("Gas used: {}", gas_used);
        println!("Transaction fee (ETH): {}", tx_fee);
        println!("Transfer transaction receipt: {} \n", json_str);

        Ok((tx_hash, gas_price, gas_used, tx_fee, json_str))
    }

    /// Mint ERC20 token(s) to an authorized address.
    /// ### Arguments
    /// * `sender` - a `Wallet` reference, the msg.sender of the mint transaction.
    /// * `to_address` - an `Address` reference, the address to mint the tokens to.
    /// Note that if the wallet is not authorized, the transaction will fail.
    /// * `amount` - a `U256` reference, the amount to mint
    /// 
    /// ### Returns
    /// `Result<(String, String, String, String, String)>` - A tuple of transaction hash, 
    /// gas price, gas used, transaction fees, and transaction receipt JSON
    pub async fn mint(
        &self,
        sender: &Wallet,
        to_address: &Address,
        amount: &U256
    ) -> Result<(String, String, String, String, String)> {
        let signer_middleware = SignerMiddleware::new(
            self.provider.clone(), 
            sender.signer.clone()
        );
        let contract_with_signer = Purse404::new(
            self.address.clone(), 
            Arc::new(signer_middleware)
        );

        let tx = contract_with_signer.mint(*to_address, *amount);
        let pending_tx = match tx.send().await {
            Ok(pending_tx) => {
                println!(
                    "Transaction sent, from: {}, to: {}, amount (wei): {} \n", 
                    to_address, 
                    self.address(), 
                    amount
                );
                println!("Waiting...");
                pending_tx
            },
            Err(e) => {
                return Err(eyre::eyre!("Failed to send transaction: {}", e))
            }
        };
        let receipt = match pending_tx.await {
            Ok(receipt) => receipt,
            Err(e) => {
                return Err(eyre::eyre!("Unexpected error occurred: {}", e))
            }
        };
        let json_str = serde_json::to_string(&receipt)?;
        let tx_hash = get_tx_hash(&json_str);
        let gas_price = get_gas_price(&json_str);
        let gas_used = get_gas_used(&json_str);
        let tx_fee = calc_tx_fee(&json_str);

        println!("Transaction hash: {}", tx_hash);
        println!("Gas price (gwei): {}", gas_price);
        println!("Gas used: {}", gas_used);
        println!("Transaction fee (ETH): {}", tx_fee);
        println!("Transfer transaction receipt: {} \n", json_str);

        Ok((tx_hash, gas_price, gas_used, tx_fee, json_str))
    }

    /// Maps "known" error signature to a human-readable string
    /// ### Arguments
    /// * `error_sig` - Error signature, eg: "0x65c62bb3"
    /// 
    /// ### Returns
    /// `String` - A human-readable string,
    /// or the original error signature if it's not known.
    pub fn map_error_sig(&self, error_sig: &str) -> String {
        match error_sig {
            "0x65c62bb3" => "InsufficientInactiveBalance()".to_string(),
            "0xab0a033b" => "IncorrectEthValue()".to_string(),
            "0x303b682f" => "MintLimitReached()".to_string(),
            _ => error_sig.to_string()
        }
    }
}
