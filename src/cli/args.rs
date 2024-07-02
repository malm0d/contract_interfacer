use crate::utils::parse_u256;
use clap::Parser;
use ethers::types::U256;

#[derive(Debug, Clone, Parser, PartialEq)]
pub struct CliArgs {
    
    /// Contract function to execute
    #[clap(long, required = true)]
    pub function: String,

    /// Executed function arguments
    #[clap(long, num_args = 1.., requires("function"))]
    pub calldata: Option<Vec<String>>,

    /// Msg.value for the function call
    #[clap(long, value_parser=parse_u256, default_value="0", requires("function"))]
    pub msg_value: U256,

    /// File path to store the csv output
    #[clap(long, required = true)]
    pub file_path: String,

    /// Derivation number
    #[clap(long, default_value_t = 0)]
    pub derivation_number: u32,


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_args() {
        let args = vec![
            "cliargs",
            "--function",
            "transfer",
            "--calldata",
            "999888777", "aaabbbccc", "0xbadc0ffee",
            "--msg-value",
            "1000000000000000000",
            "--file-path",
            "test.csv",
            "--derivation-number",
            "1",
        ];
        assert_eq!(
            CliArgs {
                function: "transfer".to_string(),
                calldata: Some(vec![
                    "999888777".to_string(),
                    "aaabbbccc".to_string(),
                    "0xbadc0ffee".to_string(),
                ]),
                msg_value: U256::from_dec_str("1000000000000000000").unwrap(),
                file_path: "test.csv".to_string(),
                derivation_number: 1,
            },
            CliArgs::try_parse_from(args).unwrap()
        );
    }

    #[test]
    fn test_cli_args_defaults() {
        let args = vec![
            "cliargs",
            "--function",
            "minted",
            "--file-path",
            "test.csv",
        ];
        assert_eq!(
            CliArgs {
                function: "minted".to_string(),
                calldata: None,
                msg_value: U256::from_dec_str("0").unwrap(),
                file_path: "test.csv".to_string(),
                derivation_number: 0,
            },
            CliArgs::try_parse_from(args).unwrap()
        );
    }
}