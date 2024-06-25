use ethers::{
    prelude::k256::ecdsa::SigningKey,
    signers::{
        MnemonicBuilder, 
        coins_bip39::English, 
        Signer, 
    },
    types::Address,
};

/// Wrapper around ethers wallet
/// With traits `Clone` and `Debug`
#[derive(Clone, Debug)]
pub struct Wallet {
    /// Signing key of the wallet
    pub signer: ethers::signers::Wallet<SigningKey>
}

/// Implementation (Methods) for Wallet
impl Wallet {
    /// Create a new wallet from the given mnemonic phrase and derivation path number
    /// #Arguments
    /// * `phrase` - Mnemonic phrase
    /// * `derivation_path_number` - Derivation path number
    /// * `chain_id` - Chain ID
    /// 
    /// #Returns
    /// `Self` - A new `Wallet` instance
    pub fn from_phrase(phrase: &str, derivation_path_number: u64, chain_id: u64) -> eyre::Result<Self> {
        let wallet_builder = MnemonicBuilder::<English>::default().phrase(phrase);
        let wallet = wallet_builder
            .derivation_path(
                format!("m/44'/60'/0'/0/{}", derivation_path_number).as_str()
            )
            .expect(
                format!("Failed to derive from phrase with path: {}", derivation_path_number).as_str()
            )
            .build()?;

        Ok(Self { signer: wallet.with_chain_id(chain_id) })
    }

    /// Generate wallets from the given number of wallets and chain ID
    /// #Arguments
    /// * `phrase` - Mnemonic phrase
    /// * `number_of_wallets` - Number of wallets to generate
    /// * `chain_id` - Chain ID
    /// 
    /// #Returns
    /// `Vec<Self>` - A vector of `Wallet` instances
    pub async fn generate_wallets(
        phrase: &str, 
        number_of_wallets: u64, 
        chain_id: u64
    ) -> eyre::Result<Vec<Self>> {
        let mut wallets = Vec::new();
        for i in 0..=number_of_wallets {
            let wallet = Wallet::from_phrase(phrase, i, chain_id)?;
            wallets.push(wallet);
        }

        Ok(wallets)
    }

    /// Returns the address of the wallet: `Address`
    pub fn address(&self) -> Address {
        self.signer.address()
    }
}
