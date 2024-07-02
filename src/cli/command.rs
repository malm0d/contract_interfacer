use super::{args::CliArgs, validate::*};
use clap::Parser;
use eyre::Result;
use ethers::types::U256;

#[derive(Debug, Parser)]
pub struct CliCommand {
    /// All Cli args
    #[clap(flatten)]
    cli_args: CliArgs,
}

impl CliCommand {
    /// Execute the command
    pub async fn execute(self) {
        let call_function = self.cli_args.function;
        let calldata = self.cli_args.calldata;
        let _ = validate_purse_calldata(&call_function, &calldata);

    }
}

// pub fn get_derivation_numbers_from_file(file_path: &str) -> Result<Vec<u32>, Box<dyn Error>> {
//     let records = read_from_csv(file_path)?;
//     let derivation_numbers = records.iter().map(|record| record.derivation).collect();
//     Ok(derivation_numbers)
// }