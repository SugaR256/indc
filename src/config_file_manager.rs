pub mod cfg {
    use serde::Deserialize;
    use std::fs;
    use std::io::ErrorKind;
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
                Err(e) => match e.kind() {
                    ErrorKind::NotFound => match fs::write("config.json", DEFAULT_CONFIG) {
                        Ok(_r) => DEFAULT_CONFIG.to_string(),
                        Err(e) => panic!(e.to_string()),
                    },
                    other_error => panic!(other_error),
                },
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
    const DEFAULT_CONFIG: &'static str = r#"{
    "FMP_API_KEY": "Get your free API key at https://financialmodelingprep.com/developer/docs/",
    "STOCKS": ["AAPL", "TSLA"]
}
"#;
}
