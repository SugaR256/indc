pub mod fmp {
    use crate::cfg;
    use reqwest::header::*;
    use std::collections::HashMap;
    #[path = "../../models/stock_profile.rs"] mod sp;
    pub struct FmpApiManager {
        pub stocks: HashMap<String, sp::StockProfile>,
        pub config: cfg::ConfigFileManager,
    }
    impl FmpApiManager {
        const FMP_URL_BASE: &'static str = "https://financialmodelingprep.com/api/v3/quote/";
        async fn call_api(&self) -> String {
            let mut headers = HeaderMap::new();
            headers.insert(UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());
            let mut stocks: String = String::new();
            for stock in self.config.stocks.iter() {
                stocks += stock;
                stocks += ",";
            }
            stocks = stocks.strip_suffix(",").unwrap().to_string();
            let stock_url = format!(
                "{base}{stocks_list}?apikey={api_key}",
                base = FmpApiManager::FMP_URL_BASE,
                stocks_list = stocks,
                api_key = self.config.fmp_api_key,
            );
            // We can't use new client with every call, it has to be stored somewehere
            let response = reqwest::Client::new()
            .get(&stock_url)
            .headers(headers)
            .send()
            .await.unwrap().text().await.unwrap();
            let is_api_key_invalid = response.contains("Invalid API KEY");
            match is_api_key_invalid {
                false => response,
                true => panic!("Invalid FinancialModelingPrep API key"),
            }
        }
        async fn parse_api_result(response: String) -> Vec<sp::StockProfile> {
            match serde_json::from_str(&response) {
                Ok(v) => v,
                Err(e) => panic!("{:#?}", e),
            }
        }
        fn stocks_vec_to_map(stocks_vec: Vec<sp::StockProfile>) -> HashMap<String, sp::StockProfile> {
            let mut map: HashMap<String, sp::StockProfile> = HashMap::new();
            for stock in stocks_vec.iter() {
                map.insert(stock.symbol.clone(), stock.clone());
            }
            map
        }
        pub async fn update_prices(
            &mut self,
        ) {
            let api_response: String = self.call_api().await;
            let received_value: Vec<sp::StockProfile> = FmpApiManager::parse_api_result(api_response).await;
            self.stocks = FmpApiManager::stocks_vec_to_map(received_value);
        }
    }
}
