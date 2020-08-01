// Is there a better way to import modules from other files?
mod config_file_manager;
use config_file_manager::cfg;
use std::collections::HashMap;

#[path = "api_managers/fmp_api_manager.rs"] mod fmp_api_manager;
use fmp_api_manager::fmp;
#[tokio::main]
async fn main() {
    let config = cfg::ConfigFileManager::construct_config();
    let mut fmp_api_manager = fmp::FmpApiManager{stocks: HashMap::new(), config: config};
    fmp_api_manager.update_prices().await;
    for stock in fmp_api_manager.stocks.iter() {
        println!("{symbol}: {price} USD", symbol = stock.0, price = stock.1.price);
    }
}
