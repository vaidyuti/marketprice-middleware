use colored::Colorize;
use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, QoS};
use std::time::Duration;
use uuid::Uuid;

use crate::topics::{EXPORT_BASE_PRICE_TOPIC, EXPORT_POWER_TOPIC};

/// Get the MQTT client.
pub(crate) fn get_client() -> (AsyncClient, EventLoop) {
    // Generate client ID.
    let uuid = Uuid::new_v4();
    let id = format!("middleware-{}", uuid);

    // Get MQTT server configuration.
    let host = dotenv::var("HOST").unwrap();
    let port = dotenv::var("PORT").unwrap().parse::<u16>().unwrap();

    // Initialise MQTT options.
    let mut options = MqttOptions::new(&id, &host, port);
    options.set_keep_alive(Duration::from_secs(5));

    info!(
        "MQTT Client {}@{}:{}",
        &id.on_white().black(),
        &host.on_bright_white().black(),
        format!("{}", &port).on_bright_yellow().black(),
    );

    // Return MQTT client.
    AsyncClient::new(options, 10)
}

/// Run the MQTT event loop and watch for messages.
pub(crate) async fn run(client: AsyncClient, mut eventloop: EventLoop) {
    let qos = QoS::AtLeastOnce;

    info!(
        "{}",
        format!(
            "Listening topics {} & {}{}...",
            &EXPORT_POWER_TOPIC.on_bright_purple().bright_yellow(),
            &EXPORT_BASE_PRICE_TOPIC.on_bright_purple().bright_yellow(),
            "".clear()
        )
        .bright_cyan()
    );

    client.subscribe(EXPORT_POWER_TOPIC, qos).await.unwrap();
    client
        .subscribe(EXPORT_BASE_PRICE_TOPIC, qos)
        .await
        .unwrap();

    loop {
        let event = eventloop.poll().await;

        if let Ok(Event::Incoming(Incoming::Publish(publish))) = event {
            debug!("{:?}", publish.payload);
        }
    }
}
