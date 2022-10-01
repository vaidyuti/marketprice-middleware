use colored::Colorize;
use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, QoS};
use std::{collections::HashMap, time::Duration};
use uuid::Uuid;

use crate::{
    seller::Seller,
    topics::{EXPORT_BASE_PRICE_TOPIC, EXPORT_IS_EXTERNAL_TOPIC, EXPORT_POWER_TOPIC},
    utils,
};

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
    client
        .subscribe(EXPORT_IS_EXTERNAL_TOPIC, qos)
        .await
        .unwrap();

    loop {
        let event = eventloop.poll().await;

        // TODO: Change to Uuid later.
        let mut prosumers: HashMap<u32, Seller> = HashMap::new();

        if let Ok(Event::Incoming(Incoming::Publish(publish))) = event {
            let topic = publish
                .topic
                .split('/')
                .map(String::from)
                .collect::<Vec<String>>();

            let prosumer_id = topic[1].parse::<u32>().unwrap();
            let value_topic = &topic[2];

            if !prosumers.contains_key(&prosumer_id) {
                prosumers.insert(prosumer_id, Seller::default());
            } else {
                // copy the object to modify.
                let mut prosumer = prosumers.get(&prosumer_id).unwrap().clone();

                match value_topic.as_str() {
                    EXPORT_BASE_PRICE_TOPIC => {
                        prosumer.export_base_price = utils::get_value::<f32>(&publish).unwrap();
                        prosumers.insert(prosumer_id, prosumer);
                    }
                    EXPORT_POWER_TOPIC => {
                        prosumer.export_power = utils::get_value::<f32>(&publish).unwrap();
                        prosumers.insert(prosumer_id, prosumer);
                    }
                    EXPORT_IS_EXTERNAL_TOPIC => {
                        prosumer.is_external = utils::get_value::<bool>(&publish).unwrap();
                        prosumers.insert(prosumer_id, prosumer);
                    }
                    _ => {
                        error!("Unknown topic {} ignoring.", value_topic);
                    }
                }
            }

            debug!("{:?} {:?}", publish.topic, publish.payload);
        }
    }
}
