use ethers::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use log::{info, warn, error, debug};

pub struct PriceMonitor {
    provider: Arc<Provider<Http>>,
    dex_addresses: HashMap<String, Address>,
    pair_contracts: HashMap<String, Contract<Arc<Provider<Http>>>>,
}

impl PriceMonitor {
    pub fn new(provider_url: &str, dex_addresses: HashMap<String, Address>) -> Self {
        let provider = Arc::new(Provider::<Http>::try_from(provider_url).expect("Failed to create provider"));
        PriceMonitor {
            provider,
            dex_addresses,
            pair_contracts: HashMap::new(),
        }
    }

    pub async fn initialize_pairs(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for (dex, pair_address) in &self.dex_addresses {
            let pair_contract = Contract::new(*pair_address, IUniswapV2Pair::abi(), self.provider.clone());
            self.pair_contracts.insert(dex.clone(), pair_contract);
        }
        info!("Pair contracts initialized successfully");
        Ok(())
    }

    pub async fn get_price(&self, dex: &str) -> Result<f64, Box<dyn std::error::Error>> {
        let pair_contract = self.pair_contracts.get(dex).ok_or("Pair contract not found")?;
        let reserves: (U256, U256, u32) = pair_contract.method("getReserves")?.call().await?;

        if reserves.0.is_zero() || reserves.1.is_zero() {
            warn!("Zero reserves detected for {}", dex);
            return Ok(0.0);
        }

        let price = reserves.0.as_u128() as f64 / reserves.1.as_u128() as f64;
        debug!("Price for {}: {}", dex, price);
        Ok(price)
    }

    pub async fn monitor_prices<F>(&self, callback: F, interval: u64)
    where
        F: Fn(HashMap<String, f64>) + Send + 'static,
    {
        loop {
            let mut prices = HashMap::new();
            for dex in self.dex_addresses.keys() {
                match self.get_price(dex).await {
                    Ok(price) => {
                        prices.insert(dex.clone(), price);
                    }
                    Err(e) => {
                        error!("Error getting price for {}: {:?}", dex, e);
                    }
                }
            }
            callback(prices);
            tokio::time::sleep(tokio::time::Duration::from_millis(interval)).await;
        }
    }
}