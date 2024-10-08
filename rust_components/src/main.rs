use rust_components::arbitrage_engine::ArbitrageEngine;
use rust_components::data_processor::DataProcessor;
use rust_components::order::Order;

use std::time::Duration;
use log::{info, error};
use env_logger;


fn main() {
    env_logger::init();

    info!("Starting the arbitrage bot");

    let mut data_processor = DataProcessor::new();
    let arbitrage_engine = ArbitrageEngine::new();

    // Simulate order book updates
    let exchange1_bids = vec![
        Order::new(100.0, 1.0).expect("Failed to create order"),
        Order::new(99.0, 2.0).expect("Failed to create order")
    ];
    let exchange1_asks = vec![
        Order::new(101.0, 1.0).expect("Failed to create order"),
        Order::new(102.0, 2.0).expect("Failed to create order")
    ];
    let exchange2_bids = vec![
        Order::new(99.5, 1.0).expect("Failed to create order"),
        Order::new(98.5, 2.0).expect("Failed to create order")
    ];
    let exchange2_asks = vec![
        Order::new(100.5, 1.0).expect("Failed to create order"),
        Order::new(101.5, 2.0).expect("Failed to create order")
    ];

    info!("Processing order book updates");
    data_processor.process_order_book_update("Exchange1", exchange1_bids.clone(), true);
    data_processor.process_order_book_update("Exchange1", exchange1_asks.clone(), false);
    data_processor.process_order_book_update("Exchange2", exchange2_bids.clone(), true);
    data_processor.process_order_book_update("Exchange2", exchange2_asks.clone(), false);

    // Run arbitrage check
    info!("Checking for arbitrage opportunities");
    match arbitrage_engine.calculate_arbitrage(
        &exchange1_bids,
        &exchange1_asks,
        &exchange2_bids,
        &exchange2_asks
    ) {
        Ok(Some(profit)) => info!("Arbitrage opportunity found! Potential profit: {}", profit),
        Ok(None) => info!("No arbitrage opportunity at this time."),
        Err(e) => error!("Error checking arbitrage: {}", e),
    }

    // Simulate continuous operation
    let duration = Duration::from_secs(10);
    info!("Running arbitrage bot for {} seconds", duration.as_secs());
    std::thread::sleep(duration);
    info!("Arbitrage bot finished running");
}
