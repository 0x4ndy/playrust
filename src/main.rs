use std::error::Error;
use std::process;
use clap::Parser;

#[derive(Parser)] struct Cli {
    url: String,
    command: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let args = Cli::parse();

    if let Err(e) = playrust::run(&args.url, &args.command).await {
        eprintln!("Application error. {e}");
        process::exit(1);
    }

    Ok(())
}
