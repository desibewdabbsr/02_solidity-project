use rust_components::arbitrage_engine::ArbitrageEngine;
use rust_components::order_book::OrderBook;
use rust_components::order::Order;
use rust_components::errors::ArbitrageEngineError;
use log::error;

#[test]
fn test_empty_order_book() {
    let book_a = OrderBook::new();
    let book_b = OrderBook::new();

    let result = ArbitrageEngine::find_arbitrage(&book_a, &book_b);
    assert!(matches!(result, Err(ArbitrageEngineError::BestBidError)), "Expected BestBidError due to empty order books");
}

#[test]
fn test_find_arbitrage_opportunity() {
    let mut book_a = OrderBook::new();
    let mut book_b = OrderBook::new();

    let order_a = Order::new(100.0, 5.0).unwrap();
    let order_b = Order::new(99.0, 5.0).unwrap();
    book_a.add_order(&order_a, true).unwrap();
    book_b.add_order(&order_b, false).unwrap();

    let result = ArbitrageEngine::find_arbitrage(&book_a, &book_b);
    
    assert!(matches!(result, Ok(Some(profit)) if profit == 1.0), "Expected arbitrage opportunity with profit of 1.0");
}

#[test]
fn test_no_arbitrage_opportunity() {
    let mut book_a = OrderBook::new();
    let mut book_b = OrderBook::new();

    let order = Order::new(99.0, 5.0).unwrap();
    book_a.add_order(&order, true).unwrap();
    if let Err(e) = book_b.add_order(&order, false) {
        error!("Failed to add order to book_b: {}", e);
    }

    let result = ArbitrageEngine::find_arbitrage(&book_a, &book_b);

    assert!(matches!(result, Ok(None)), "Expected no arbitrage opportunity, but found one");
}

#[test]
fn test_calculate_arbitrage() {
    let engine = ArbitrageEngine::new();
    
    let bids_a = vec![Order::new(100.0, 5.0).unwrap()];
    let asks_a = vec![Order::new(101.0, 5.0).unwrap()];
    let bids_b = vec![Order::new(99.0, 5.0).unwrap()];
    let asks_b = vec![Order::new(98.0, 5.0).unwrap()];

    let result = engine.calculate_arbitrage(&bids_a, &asks_a, &bids_b, &asks_b);
    
    assert!(matches!(result, Ok(Some(profit)) if profit == 2.0), "Expected arbitrage opportunity with profit of 2.0");
}
