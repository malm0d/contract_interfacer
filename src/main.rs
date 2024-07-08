#[tokio::main]
async fn main() {
    if let Err(err) = contract_interfacer::cli::run() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}


//     let purse_token = PurseToken404Contract::new(
//         to_address_type(PURSE_ETH_ADDRESS),
//         &Arc::new(prov.clone()),
//     );

//     let sender_eth_bal_bef = get_native_balance(&prov, &msg_sender_address).await.unwrap();
//     let sender_erc20_bal_bef = purse_token.balance_of(&msg_sender_address).await.unwrap();
//     let recipient_eth_bal_bef = get_native_balance(&prov, &msg_recipient_address).await.unwrap();
//     let recipient_erc20_bal_bef = purse_token.balance_of(&msg_recipient_address).await.unwrap();

//     let transfer_receipt = purse_token.transfer(
//         &wallet,
//         &msg_recipient_address,
//         &calldata_value
//     ).await;

//     match transfer_receipt {
//         Ok(receipt) => {
//             let (
//                 tx_hash, 
//                 gas_price, 
//                 gas_used, 
//                 tx_fees, 
//                 tx_receipt_json
//             ) = receipt;

//             let msg_sender_owned_token_ids = purse_token.owned(&wallet.address()).await.unwrap();

//             let sender_eth_bal_aft = get_native_balance(&prov, &msg_sender_address).await.unwrap();
//             let sender_erc20_bal_aft = purse_token.balance_of(&msg_sender_address).await.unwrap();
//             let recipient_eth_bal_aft = get_native_balance(&prov, &msg_recipient_address).await.unwrap();
//             let recipient_erc20_bal_aft = purse_token.balance_of(&msg_recipient_address).await.unwrap();

//             let _ = write_to_csv(
//                 file_path,
//                 &tx_hash,
//                 &gas_price,
//                 &gas_used,
//                 &tx_fees,
//                 &tx_receipt_json,
//                 call_function,
//                 &derivation_num,
//                 &msg_sender_address,
//                 Some(sender_eth_bal_bef),
//                 Some(sender_eth_bal_aft),
//                 Some(sender_erc20_bal_bef),
//                 Some(sender_erc20_bal_aft),
//                 &msg_recipient_address,
//                 Some(recipient_eth_bal_bef),
//                 Some(recipient_eth_bal_aft),
//                 Some(recipient_erc20_bal_bef),
//                 Some(recipient_erc20_bal_aft),
//                 Some(msg_value),
//                 Some(calldata_value),
//                 Some(msg_sender_owned_token_ids)
//             );
//         },
//         Err(e) => {
//             println!("Error: {:?}", e);
//             panic!("Terminating program");
//         }
//     }
// }
