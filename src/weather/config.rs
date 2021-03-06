use config::Config;

/// This struct defines a required configuration for the Weather Actor.
pub struct Configuration {
    // Airly.eu's API key
    pub key: String,

    // Id of sensor from which all data will be fetched
    pub sensor_id: u32,
}

impl Configuration {
    pub fn new(config: &mut Config) -> Configuration {
        Configuration {
            key: config.get("apis.airly.key").unwrap(),
            sensor_id: config.get("apis.airly.sensor_id").unwrap_or(0),
        }
    }

    /// Returns `true` if configuration's API key has been set.
    pub fn is_set(&self) -> bool {
        return self.key.len() > 0;
    }
}