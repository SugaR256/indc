pub mod cfg {
    use serde::Deserialize;
    use std::fs;
    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub struct ConfigFileManager {
        pub fmp_api_key: String,
        pub stocks: Vec<String>,
    }
    impl ConfigFileManager {
        fn read_config_file() -> String {
            match fs::read_to_string("config.json") {
                Ok(result) => result,
                Err(e) => panic!(e.to_string()),
            }
        }
        pub fn construct_config() -> ConfigFileManager {
            let config_text: String = ConfigFileManager::read_config_file();
            match serde_json::from_str(&config_text) {
                Ok(config_file) => config_file,
                Err(e) => panic!(e.to_string()),
            }
        }
    }
}
