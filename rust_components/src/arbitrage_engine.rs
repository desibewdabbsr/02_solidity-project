use log::{error, info};
use crate::order_book::OrderBook;
use crate::errors::ArbitrageEngineError;
use crate::order::Order;

pub struct ArbitrageEngine;

impl ArbitrageEngine {
    const MINIMUM_PROFIT_THRESHOLD: f64 = 0.01;

    pub fn new() -> Self {
        ArbitrageEngine
    }

    pub fn calculate_arbitrage(
        &self,
        exchange1_bids: &[Order],
        exchange1_asks: &[Order],
        exchange2_bids: &[Order],
        exchange2_asks: &[Order]
    ) -> Result<Option<f64>, String> {
        let book_a = OrderBook::from_orders(exchange1_bids.to_vec(), exchange1_asks.to_vec());
        let book_b = OrderBook::from_orders(exchange2_bids.to_vec(), exchange2_asks.to_vec());
    
        match Self::find_arbitrage(&book_a, &book_b) {
            Ok(opportunity) => Ok(opportunity),
            Err(e) => Err(e.to_string()),
        }
    }
    


    pub fn find_arbitrage(
        book_a: &OrderBook,
        book_b: &OrderBook
    ) -> Result<Option<f64>, ArbitrageEngineError> {
        info!("Starting arbitrage detection between two order books");

        let best_bid_a = book_a.get_best_bid()
            .map(|(price, _)| price)
            .ok_or_else(|| {
                error!("Failed to retrieve the best bid from book A");
                ArbitrageEngineError::BestBidError
            })?;

        let best_ask_b = book_b.get_best_ask()
            .map(|(price, _)| price)
            .ok_or_else(|| {
                error!("Failed to retrieve the best ask from book B");
                ArbitrageEngineError::BestAskError
            })?;

        info!(
            "Best bid from book A: {}, Best ask from book B: {}",
            best_bid_a, best_ask_b
        );

        if best_bid_a > best_ask_b {
            let profit = best_bid_a - best_ask_b;
            info!(
                "Potential arbitrage opportunity found. Profit: {}",
                profit
            );

            if profit > Self::MINIMUM_PROFIT_THRESHOLD {
                info!(
                    "Arbitrage opportunity confirmed with profit above threshold: {}",
                    profit
                );
                Ok(Some(profit))
            } else {
                info!(
                    "Potential arbitrage opportunity discarded due to insufficient profit. Profit: {}",
                    profit
                );
                Ok(None)
            }
        } else {
            info!("No arbitrage opportunity found as best bid is not greater than best ask.");
            Ok(None)
        }
    }
}
