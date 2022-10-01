use colored::Colorize;
use std::env;

#[macro_use]
extern crate log;

mod client;
mod topics;
mod seller;
mod utils;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    info!(
        "{} {} version {}",
        "Vaidyuti".bright_cyan(),
        "Marketprice Middleware".yellow(),
        env!("CARGO_PKG_VERSION")
    );

    info!("{}", "Reading .env...".bright_green());
    dotenv::from_filename(".env").ok();

    let (client, eventloop) = client::get_client();
    client::run(client, eventloop).await;
}
