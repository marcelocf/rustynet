use rustynet::echonet::discovery::find;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> rustynet::Result<()> {
    tracing_subscriber::fmt::init();
    info!("Scanning for devices");
    find().expect("Unable to find devices");
    Ok(())
}
