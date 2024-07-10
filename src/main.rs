#[tokio::main]
async fn main() {
    if let Err(err) = contract_interfacer::cli::run() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}