use crate::price_monitor::PriceMonitor;
use std::collections::HashMap;
use log::{info, debug};

pub struct ArbitrageFinder {
    price_monitor: PriceMonitor,
    min_profit_threshold: f64,
}

pub struct ArbitrageOpportunity {
    pub buy_dex: String,
    pub sell_dex: String,
    pub profit: f64,
}

impl ArbitrageFinder {
    pub fn new(price_monitor: PriceMonitor, min_profit_threshold: f64) -> Self {
        ArbitrageFinder {
            price_monitor,
            min_profit_threshold,
        }
    }

    pub async fn monitor_for_arbitrage_opportunities<F>(&self, callback: F)
    where
        F: Fn(ArbitrageOpportunity) + Send + 'static,
    {
        self.price_monitor.monitor_prices(move |prices| {
            if let Some(opportunity) = self.find_arbitrage_opportunity(&prices) {
                callback(opportunity);
            }
        }, 10000).await;
    }

    fn find_arbitrage_opportunity(&self, prices: &HashMap<String, f64>) -> Option<ArbitrageOpportunity> {
        let mut best_opportunity = None;
        let mut max_profit = 0.0;

        for (buy_dex, buy_price) in prices {
            for (sell_dex, sell_price) in prices {
                if buy_dex != sell_dex {
                    let profit = sell_price - buy_price;
                    if profit > max_profit && profit > self.min_profit_threshold {
                        max_profit = profit;
                        best_opportunity = Some(ArbitrageOpportunity {
                            buy_dex: buy_dex.clone(),
                            sell_dex: sell_dex.clone(),
                            profit,
                        });
                    }
                }
            }
        }

        if let Some(ref opportunity) = best_opportunity {
            info!("Arbitrage opportunity found: Buy from {} at {}, Sell to {} at {}, Profit: {}",
                  opportunity.buy_dex, prices[&opportunity.buy_dex],
                  opportunity.sell_dex, prices[&opportunity.sell_dex],
                  opportunity.profit);
        } else {
            debug!("No arbitrage opportunity found");
        }

        best_opportunity
    }
}