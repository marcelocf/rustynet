use rustynet::echonet::discovery::find;
use tracing::info;
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt::init();
    find().expect("Unable to find devices");
    info!("Hello, world!");
}
