use const_format::concatcp;

/// MQTT topic of prosumers.
pub const PROSUMER_BASE_TOPIC: &str = "prosumers/+";

/// MQTT topic with `export_power` value.
pub const EXPORT_POWER_TOPIC: &str = concatcp!(PROSUMER_BASE_TOPIC, "/export_power");

/// MQTT topic with `export_base_price` value.
pub const EXPORT_BASE_PRICE_TOPIC: &str = concatcp!(PROSUMER_BASE_TOPIC, "/export_base_price");

/// MQTT topic with `export_base_price` value.
pub const EXPORT_IS_EXTERNAL_TOPIC: &str = concatcp!(PROSUMER_BASE_TOPIC, "/is_external");
