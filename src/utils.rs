use std::str::FromStr;

use rumqttc::Publish;

/// Get value of type `T` from `Publish`.
pub fn get_value<T: FromStr>(publish: &Publish) -> Result<T, <T as FromStr>::Err> {
    // Convert `Bytes` to `Vec<u8>`.
    let mut bytes = Vec::new();
    for byte in publish.payload.iter() {
        bytes.push(*byte)
    }
    // Get `String` message and parse float value.
    let values = String::from_utf8(bytes).unwrap();
    values.parse::<T>()
}
