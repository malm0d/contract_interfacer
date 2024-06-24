use csv::Writer;
use eyre::Result;
use serde_json::Value;
use std::{
    fs, str::FromStr, sync::Arc
};
use ethers::{
    prelude::SignerMiddleware, 
    providers::Middleware, 
    types::{
        Address, H256, U256,
    },
    contract::abigen
};
use crate::wallet::Wallet;

abigen!(
    PurseToken404,
    "abi/purseTokenAbi.json",
);

/// Wrapper around PurseToken404 contract
/// With traits `Clone` and `Debug`
/// Fields:
/// * `address` - Address in `Address` type
/// * `contract` - PurseToken404 contract instance
/// * `provider` - Provider
#[derive(Clone, Debug)]
pub struct PurseToken404Contract<M: Middleware + 'static> {
    address: Address,
    contract: PurseToken404<M>,
    provider: Arc<M>,
}

impl<M: Middleware + 'static> PurseToken404Contract<M> {
    /// Create a new `PurseToken404Contract` instance
    pub fn new(address: Address, provider: Arc<M>) -> Self {
        let contract = PurseToken404::new(address, provider.clone());
        Self { address, contract, provider }
    }

    /// Return the address of the contract
    pub fn address(&self) -> Address {
        self.address
    }

    /// Return an instance of the provider
    pub fn provider(&self) -> Arc<M> {
        self.provider.clone()
    }

    /// Return the balance of the given address
    /// #Arguments
    /// * `addr` - Address
    /// 
    /// #Returns
    /// `Result<U256>` - A `U256` type
    pub async fn balance_of(&self, addr: Address) -> Result<U256> {
        let res = self.contract.balance_of(addr).call().await;
        match res {
            Ok(balance) => Ok(balance),
            Err(e) => Err(eyre::Report::from(e))
        }
    }

    /// Return the current minted NFT amount
    /// 
    /// #Returns
    /// `Result<U256>` - A `U256` type
    pub async fn minted(&self) -> Result<U256> {
        let res = self.contract.minted().call().await;
        match res {
            Ok(minted) => Ok(minted),
            Err(e) => Err(eyre::Report::from(e))
        }
    }

    /// Return the current minting cost to mint an NFT
    /// 
    /// #Returns
    /// `Result<U256>` - A `U256` type
    pub async fn minting_cost(&self) -> Result<U256> {
        let res = self.contract.minting_cost().call().await;
        match res {
            Ok(cost) => Ok(cost),
            Err(e) => Err(eyre::Report::from(e))
        }
    }

    /// Retrieves all NFT token IDs owned by the given address
    /// #Arguments
    /// * `owner` - Address
    /// 
    /// #Returns
    /// `Result<Vec<U256>>` - A vector of `U256` types
    pub async fn owned(&self, owner: Address) -> Result<Vec<U256>> {
        let res = self.contract.owned(owner).call().await;
        match res {
            Ok(owned) => Ok(owned),
            Err(e) => Err(eyre::Report::from(e))
        }
    }

    /// Transfer the given amount (ERC20), from a `Wallet` to the given address.
    /// The completed transaction will be recorded in a CSV file
    /// #Arguments
    /// * `from` - Wallet, the sender of the transfer
    /// * `to` - Address, the recipient of the transfer
    /// * `amount` - U256, the amount to transfer
    /// 
    /// #Returns
    /// `Result<()>` - An empty `Result`
    pub async fn transfer(&self, from: Wallet, to: Address, amount: U256) -> Result<()> {
        let signer_middleware = SignerMiddleware::new(
            self.provider.clone(),
            from.signer.clone()
        );
        let contract_with_signer = PurseToken404::new(
            self.address.clone(),
            Arc::new(signer_middleware)
        );

        println!("Test1");
        let tx = contract_with_signer.transfer(to, amount);
        println!("Test2");
        let pending = tx.send().await;
        match pending {
            Ok(_) => println!("Pending"),
            Err(e) => println!("Error: {}", e)
        }
        println!("Test3");
        // let finalized = pending;

        // let json_str = serde_json::to_string(finalized)?;
        // let json: Value = serde_json::from_str(&json_str)?;

        // println!("Transfer transaction receipt: {}", json_str);

        Ok(())
    }

    /// Mint an ERC721 token to the given wallet.
    /// The completed transaction will be recorded in a CSV file
    /// #Arguments
    /// * `mint_unit` - U256, the amount to mint
    /// * `wallet` - Wallet, the wallet to mint the NFT to.
    /// 
    /// #Returns
    /// `Result<()>` - An empty `Result`
    pub async fn mint_erc721(&self, mint_unit: U256, wallet: Wallet) -> Result<()> {
        let minting_cost = self.minting_cost().await?;
        let signer_middleware = SignerMiddleware::new(
            self.provider.clone(), 
            wallet.signer.clone()
        );
        let contract_with_signer = PurseToken404::new(
            self.address.clone(), 
            Arc::new(signer_middleware)
        );

        let tx = contract_with_signer.mint_erc721(mint_unit).value(minting_cost);
        let pending = tx.send().await?;
        let finalized = pending.await?;

        let json_str = serde_json::to_string(&finalized)?;
        let json: Value = serde_json::from_str(&json_str)?;

        println!("Mint transaction receipt: {}", serde_json::to_string(&finalized)?);

        let token_id_vec = self.owned(wallet.address()).await?;
        let token_id = token_id_vec[0];

        if let Some(tx_hash) = json["transactionHash"].as_str() {
            let file_path = "../transaction_receipts.csv";
            let file = fs::File::create(file_path).expect("Unable to create file");
            let mut writer = Writer::from_writer(file);

            if fs::metadata(file_path).is_err() {
                writer.write_record(
                    &["Address", "Transaction Hash", "Minted", "Token ID"]
                )?;
            }

            writer.write_record(
                &[
                    wallet.address().to_string(),
                    String::from_str(tx_hash)?,
                    "true".to_string(),
                    token_id.to_string()
                ]
            ).expect("Could not write to file");

            writer.flush()?;
            println!("Transaction hash: {} added to file", tx_hash);
        } else {
            println!("Transaction hash not found");
        }

        Ok(())
    }


}

// /// Create an instance of the contract
// /// #Arguments
// /// * `provider` - Provider
// /// 
// /// #Returns
// /// `PurseToken404<Provider<Http>>` - A new instance of `PurseToken404<Provider<Http>>`
// pub async fn get_pursetoken404_contract(
//     provider: Provider<Http>
// ) -> eyre::Result<PurseToken404<Provider<Http>>> {
//     Ok(PurseToken404::new(
//         PURSE_ETH_ADDRESS.parse::<Address>()?,
//         Arc::new(provider.clone())
//     ))
// }

// pub async fn mint_nft(wallet: Wallet, provider: Provider<Http>) -> eyre::Result<(), Box<dyn std::error::Error>> {
//     let contract = get_pursetoken404_contract(provider).await?;

//     Ok(())
// }
