use contract_interfacer::{
    Wallet,
    PurseToken404Contract,
    PURSE_ETH_ADDRESS,
    get_provider, to_address_type, to_u256, write_to_csv
};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let phrase = std::env::var("MNEMONIC").unwrap();
    let wallet = Wallet::from_phrase(phrase.as_str(), 0, 11155111).unwrap();

    let prov = get_provider(
        std::env::var("SEPOLIA_RPC").unwrap().as_str()
    ).await.unwrap().into();

    let purse_token = PurseToken404Contract::new(
        to_address_type(PURSE_ETH_ADDRESS),
        prov
    );

    let owned_token_ids = purse_token.owned(&wallet.address()).await.unwrap();
    println!("NFTs: {:?}", owned_token_ids);

    // let wallet_balance = purse_token.balance_of(&wallet.address()).await.unwrap();
    // println!("{:?}", wallet_balance);

    // let mint_unit = to_u256(1);
    // let mint_to = wallet.address();
    // let res = purse_token.mint_erc721(
    //     &mint_unit, 
    //     &wallet
    // ).await;

    // match res {
    //     Ok(receipt) => {println!("ok")},
    //     Err(e) => { println!("Error: {:?}", e) }
    // }

    // let result = purse_token.transfer(
    //     &wallet,
    //     &to_address_type("0xdF7eD90AC34a1492fD0240ea385bab6872a96527"),
    //     &amount
    // ).await;

    // match result {
    //     Ok(receipt) => {println!("ok")},
    //     Err(e) => { println!("Error: {:?}", e) }
    // }

}
