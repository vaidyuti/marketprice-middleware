use std::time::Duration;

use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, QoS};
use uuid::Uuid;

/// MQTT topic with `export_power` value.
const EXPORT_POWER_TOPIC: &str = "prosumers/+/export_power";

/// MQTT topic with `export_base_price` value.
const EXPORT_BASE_PRICE_TOPIC: &str = "prosumers/+/export_base_price";

/// Get the MQTT client.
pub(crate) fn get_client() -> (AsyncClient, EventLoop) {
    // Generate client ID.
    let uuid = Uuid::new_v4();
    let id = format!("middleware-{}", uuid);

    // Get MQTT server configuration.
    let host = dotenv::var("HOST").unwrap();
    let port = dotenv::var("PORT").unwrap().parse::<u16>().unwrap();

    // Initialise MQTT options.
    let mut options = MqttOptions::new(id, host, port);
    options.set_keep_alive(Duration::from_secs(5));

    // Return MQTT client.
    AsyncClient::new(options, 10)
}

/// Run the MQTT event loop and watch for messages.
pub(crate) async fn run(client: AsyncClient, mut eventloop: EventLoop) {
    let qos = QoS::AtLeastOnce;

    client.subscribe(EXPORT_POWER_TOPIC, qos).await.unwrap();
    client
        .subscribe(EXPORT_BASE_PRICE_TOPIC, qos)
        .await
        .unwrap();

    loop {
        let event = eventloop.poll().await;

        if let Ok(Event::Incoming(Incoming::Publish(publish))) = event {
            println!("{:?}", publish.payload);
        }
    }
}
