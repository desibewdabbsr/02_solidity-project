use rust_components::arbitrage_engine::ArbitrageEngine;
use rust_components::data_processor::DataProcessor;
use rust_components::order::Order;



#[test]
fn test_main_functionality() {
    let mut data_processor = DataProcessor::new();
    let arbitrage_engine = ArbitrageEngine::new();

    let exchange1_bids = vec![Order::new(100.0, 1.0).unwrap()];
    let exchange1_asks = vec![Order::new(101.0, 1.0).unwrap()];
    let exchange2_bids = vec![Order::new(102.0, 1.0).unwrap()];
    let exchange2_asks = vec![Order::new(99.0, 1.0).unwrap()];

    data_processor.process_order_book_update("Exchange1", exchange1_bids.clone(), true);
    data_processor.process_order_book_update("Exchange1", exchange1_asks.clone(), false);
    data_processor.process_order_book_update("Exchange2", exchange2_bids.clone(), true);
    data_processor.process_order_book_update("Exchange2", exchange2_asks.clone(), false);

    let result = arbitrage_engine.calculate_arbitrage(
        &exchange1_bids,
        &exchange1_asks,
        &exchange2_bids,
        &exchange2_asks
    );

    assert!(result.is_ok(), "Arbitrage calculation should succeed");
    assert!(result.unwrap().is_some(), "An arbitrage opportunity should be found");
}
