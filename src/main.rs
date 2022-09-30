use anyhow::Result;
use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};
use std::time::Duration;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::from_filename(".env").ok();

    let uuid = Uuid::new_v4();
    let id = format!("middleware-{}", uuid);
    let host = dotenv::var("HOST").unwrap();
    let port = dotenv::var("PORT").unwrap().parse::<u16>().unwrap();

    let mut mqtt_options = MqttOptions::new(id, host, port);
    mqtt_options.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);

    client
        .subscribe("prosumers/+/export_power", QoS::AtMostOnce)
        .await
        .unwrap();

    loop {
        let event = eventloop.poll().await;

        if let Ok(Event::Incoming(Incoming::Publish(publish))) = event {
            println!("{:?}", publish.payload);
        }
    }
}
