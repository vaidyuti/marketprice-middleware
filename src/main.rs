mod client;

#[tokio::main]
async fn main() {
    dotenv::from_filename(".env").ok();

    let (client, eventloop) = client::get_client();
    client::run(client, eventloop).await;
}
