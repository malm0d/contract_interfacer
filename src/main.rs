use contract_interfacer::{
    Wallet,
    PurseToken404Contract,
    PURSE_ETH_ADDRESS,
    get_provider, to_address_type
};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let phrase = std::env::var("MNEMONIC").unwrap();
    let wallet = Wallet::from_phrase(phrase.as_str(), 0, 1).unwrap();
    println!("{:?}", wallet);

    let prov = get_provider(
        std::env::var("SEPOLIA_RPC").unwrap().as_str()
    ).await.unwrap().into();

    let purse_token = PurseToken404Contract::new(
        to_address_type(PURSE_ETH_ADDRESS),
        prov
    );

    println!("{:?}", purse_token);
    
    let contract_minted = purse_token.minted().await.unwrap();
    println!("{:?}", contract_minted);


}
