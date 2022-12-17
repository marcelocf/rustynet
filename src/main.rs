use rustynet::discovery::find;
use tracing::info;
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt::init();
    find().expect("omg");
    info!("Hello, world!");
}
