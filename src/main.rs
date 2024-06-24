use contract_interfacer::{
    Wallet,
    PurseToken404Contract,
    PURSE_ETH_ADDRESS,
    get_provider, to_address_type, to_u256
};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let phrase = std::env::var("MNEMONIC").unwrap();
    let wallet = Wallet::from_phrase(phrase.as_str(), 0, 11155111).unwrap();
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

    let token_ids = purse_token.owned(wallet.address()).await.unwrap();
    println!("{:?}", token_ids);

    let wallet_balance = purse_token.balance_of(wallet.address()).await.unwrap();
    println!("{:?}", wallet_balance);

    let amount = to_u256(1);
    let _ = purse_token.transfer(
        wallet,
        to_address_type("0x2027E055201E26b1bFE33Eb923b3fdb7E6f30807"),
        amount
    ).await;


}
