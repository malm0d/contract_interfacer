use clap::{Parser, Subcommand};
use std::panic;

pub mod args;
pub mod commands;

/// Main CLI interface
#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands
}

/// Commands to be executed
#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name = "purse")]
    Purse(commands::PurseCommand),
}

pub fn run()  -> eyre::Result<()> {
    let cli = Cli::parse();
    let stack_size = 16 * 1024 * 1024; //16 MB

    std::thread::Builder::new()
        .stack_size(stack_size) //16 MB
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .thread_stack_size(stack_size)
                .build()?;

            let task = async move {
                match cli.command {
                    Commands::Purse(command) => command.execute().await,
                }
            };
            
            rt.block_on(task)?;
            Ok(())
        })?
        .join()
        .unwrap_or_else(|e| panic::resume_unwind(e))

}