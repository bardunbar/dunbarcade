use configparser::ini::Ini;
use macroquad::prelude::*;
use std::collections::HashMap as Map;

struct ConfigMap(Map<String, Map<String, Option<String>>>);

pub struct ConfigSettings {
    config: Map<String, ConfigMap>
}

impl ConfigSettings {
    pub fn new() -> Self {
        ConfigSettings {
            config: Map::new(),
        }
    }

    pub async fn load_ini(&mut self, config_key: &str, path: &str) {
        fn load_ini_internal(config_settings: &mut ConfigSettings, config_key: &str, contents: String) {
            let mut config_reader = Ini::new();
            match config_reader.read(contents) {
                Ok(data) => {
                    config_settings.config.insert(config_key.to_owned(), ConfigMap(data));
                }
                Err(msg) => {error!("Ini Parse Error: {:?}", msg)}
            }
        }

        match load_string(path).await {
            Ok(contents) => {
                load_ini_internal(self, config_key, contents);
            },
            Err(msg) => {error!("File Error: {:?}", msg)}
        }
    }

    pub fn get_int(&self, config_key: &str, category: &str, key: &str) -> Result<i64, String> {
        match self.config.get(&config_key.to_owned()) {
            Some(config_map) => match config_map.0.get(&category.to_owned()) {
                Some(section) => match section.get(&key.to_owned()){
                    Some(val) => match val {
                        Some(inner) => match inner.parse::<i64>() {
                            Ok(int) => Ok(int),
                            Err(why) => Err(format!("Unable to parse config value ({}/{}/{}) as int: {}", config_key, category, key, why)),
                        },
                        None => Err(format!("Unable to find config value ({}/{}) within config: {}", category, key, config_key))
                    },
                    None => Err(format!("Unable to find config value ({}/{}) within config: {}", category, key, config_key))
                },
                None => Err(format!("Unable to find section ({}) within config: {}", category, config_key))
            },
            None => Err(format!("Unable to find config by key: {}", config_key))
        }
    }

}