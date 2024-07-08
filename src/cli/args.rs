use crate::utils::parse_u256;
use clap::Parser;
use ethers::types::U256;

#[derive(Debug, Clone, Parser, PartialEq)]
pub struct ContractCliArgs {
    /// Hueristic Derivation number
    #[clap(long, default_value_t = 0)]
    pub derivation_number: u32,
    
    /// Contract function to execute
    #[clap(long, required = true)]
    pub function: String,

    /// Executed function arguments
    #[clap(long, num_args = 1.., requires("function"))]
    pub calldata: Option<Vec<String>>,

    /// Msg.value for the function call
    #[clap(long, value_parser=parse_u256, default_value="0", requires("function"))]
    pub msg_value: U256,

    /// Chain Id: 1 for mainnet, 11155111 for sepolia
    #[clap(long, required = true)]
    pub chain_id: u32,

    /// File path to store the csv output
    #[clap(long, required = true)]
    pub file_path: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_args() {
        let args = vec![
            "ContractCliArgs",
            "--derivation-number",
            "1",
            "--function",
            "transfer",
            "--calldata",
            "999888777", "aaabbbccc", "0xbadc0ffee",
            "--msg-value",
            "1000000000000000000",
            "--chain-id",
            "1",
            "--file-path",
            "test.csv",
        ];
        assert_eq!(
            ContractCliArgs {
                derivation_number: 1,
                function: "transfer".to_string(),
                calldata: Some(vec![
                    "999888777".to_string(),
                    "aaabbbccc".to_string(),
                    "0xbadc0ffee".to_string(),
                ]),
                msg_value: U256::from_dec_str("1000000000000000000").unwrap(),
                chain_id: 1,
                file_path: "test.csv".to_string(),
            },
            ContractCliArgs::try_parse_from(args).unwrap()
        );
    }

    #[test]
    fn test_cli_args_2() {
        let args = vec![
            "ContractCliArgs",
            "--derivation-number",
            "1",
            "--function",
            "transfer",
            "--calldata",
            "999888777", "aaabbbccc", "0xbadc0ffee",
            "--msg-value",
            "1000000000000000000",
            "--chain-id",
            "11155111",
            "--file-path",
            "test.csv",
        ];
        assert_eq!(
            ContractCliArgs {
                derivation_number: 1,
                function: "transfer".to_string(),
                calldata: Some(vec![
                    "999888777".to_string(),
                    "aaabbbccc".to_string(),
                    "0xbadc0ffee".to_string(),
                ]),
                msg_value: U256::from_dec_str("1000000000000000000").unwrap(),
                chain_id: 11155111,
                file_path: "test.csv".to_string(),
            },
            ContractCliArgs::try_parse_from(args).unwrap()
        );
    }

    #[test]
    fn test_cli_args_3() {
        let args = vec![
            "ContractCliArgs",
            "--function",
            "minted",
            "--chain-id",
            "11155111",
            "--file-path",
            "test.csv",
        ];
        assert_eq!(
            ContractCliArgs {
                derivation_number: 0,
                function: "minted".to_string(),
                calldata: None,
                msg_value: U256::from_dec_str("0").unwrap(),
                chain_id: 11155111,
                file_path: "test.csv".to_string(),
            },
            ContractCliArgs::try_parse_from(args).unwrap()
        );
    }
}