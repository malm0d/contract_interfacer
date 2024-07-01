use std::sync::Arc;
use contract_interfacer::{
    Wallet,
    PurseToken404Contract,
    PURSE_ETH_ADDRESS,
    get_provider, 
    to_address_type, 
    to_u256, 
    get_native_balance,
    write_to_csv
};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let file_path = "test.csv";
    let phrase = std::env::var("MNEMONIC").unwrap();
    let derivation_num = 0;
    let wallet = Wallet::from_phrase(phrase.as_str(), derivation_num, 11155111).unwrap();
    let msg_sender_address = wallet.address();
    let msg_recipient_address = to_address_type("0xdF7eD90AC34a1492fD0240ea385bab6872a96527");
    
    let calldata_value = to_u256(1); //1 wei
    let msg_value = to_u256(10_000_000_000_000_000); //0.01 eth

    let call_function = "transfer";
    // let call_function = "mintERC721";

    let prov = get_provider(
        std::env::var("SEPOLIA_RPC").unwrap().as_str()
    ).await.unwrap();

    let purse_token = PurseToken404Contract::new(
        to_address_type(PURSE_ETH_ADDRESS),
        &Arc::new(prov.clone()),
    );

    let sender_eth_bal_bef = get_native_balance(&prov, &msg_sender_address).await.unwrap();
    let sender_erc20_bal_bef = purse_token.balance_of(&msg_sender_address).await.unwrap();
    let recipient_eth_bal_bef = get_native_balance(&prov, &msg_recipient_address).await.unwrap();
    let recipient_erc20_bal_bef = purse_token.balance_of(&msg_recipient_address).await.unwrap();

    let transfer_receipt = purse_token.transfer(
        &wallet,
        &msg_recipient_address,
        &calldata_value
    ).await;

    match transfer_receipt {
        Ok(receipt) => {
            let (
                tx_hash, 
                gas_price, 
                gas_used, 
                tx_fees, 
                tx_receipt_json
            ) = receipt;

            let msg_sender_owned_token_ids = purse_token.owned(&wallet.address()).await.unwrap();

            let sender_eth_bal_aft = get_native_balance(&prov, &msg_sender_address).await.unwrap();
            let sender_erc20_bal_aft = purse_token.balance_of(&msg_sender_address).await.unwrap();
            let recipient_eth_bal_aft = get_native_balance(&prov, &msg_recipient_address).await.unwrap();
            let recipient_erc20_bal_aft = purse_token.balance_of(&msg_recipient_address).await.unwrap();

            let _ = write_to_csv(
                file_path,
                &tx_hash,
                &gas_price,
                &gas_used,
                &tx_fees,
                &tx_receipt_json,
                call_function,
                &derivation_num,
                &msg_sender_address,
                Some(sender_eth_bal_bef),
                Some(sender_eth_bal_aft),
                Some(sender_erc20_bal_bef),
                Some(sender_erc20_bal_aft),
                &msg_recipient_address,
                Some(recipient_eth_bal_bef),
                Some(recipient_eth_bal_aft),
                Some(recipient_erc20_bal_bef),
                Some(recipient_erc20_bal_aft),
                Some(msg_value),
                Some(calldata_value),
                Some(msg_sender_owned_token_ids)
            );
        },
        Err(e) => {
            println!("Error: {:?}", e);
            panic!("Terminating program");
        }
    }
}
