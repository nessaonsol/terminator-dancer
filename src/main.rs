use anyhow::Result;
use clap::Parser;
use terminator_dancer::TerminatorRuntime;

#[derive(Parser, Debug)]
#[clap(name = "Terminator-Dancer", version = "0.1.0", about = "A lightweight Solana runtime")]
struct Args {
    #[clap(short, long, default_value = "config.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let runtime = TerminatorRuntime::new(&args.config).await?;
    runtime.start().await?;
    Ok(())
}
