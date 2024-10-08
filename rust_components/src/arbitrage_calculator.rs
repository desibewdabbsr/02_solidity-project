use log::{info, warn};

pub struct ArbitrageCalculator;

impl ArbitrageCalculator {
    pub fn calculate_arbitrage(best_bid: Option<f64>, best_ask: Option<f64>) -> Option<f64> {
        info!("Calculating arbitrage between best bid and best ask");

        match (best_bid, best_ask) {
            (Some(bid), Some(ask)) => {
                if bid > ask {
                    let profit = bid - ask;
                    info!("Arbitrage opportunity found: profit = {}", profit);
                    Some(profit)
                } else {
                    warn!("No arbitrage opportunity found. Bid ({}) <= Ask ({})", bid, ask);
                    None
                }
            }
            _ => {
                warn!("Unable to calculate arbitrage due to missing bid or ask");
                None
            }
        }
    }
}
