use rust_components::main_module::ArbitrageBot;
use rust_components::order::Order;
use std::time::Duration;

#[test]
fn test_arbitrage_bot() {
    let bot = ArbitrageBot::new();
    assert!(bot.check_arbitrage("Exchange1", "Exchange2").is_err());
}

#[test]
fn test_update_order_books() {
    let mut bot = ArbitrageBot::new();
    let exchange = "Exchange1";
    let bids = vec![Order::new(100.0, 1.0).unwrap()];
    let asks = vec![Order::new(101.0, 1.0).unwrap()];

    bot.update_order_books(exchange, bids.clone(), asks.clone());

    let order_book = bot.get_order_book(exchange).unwrap();
    assert_eq!(order_book.0, bids);
    assert_eq!(order_book.1, asks);
}

#[test]
fn test_get_order_book() {
    let mut bot = ArbitrageBot::new();
    let exchange = "Exchange1";
    let bids = vec![Order::new(100.0, 1.0).unwrap()];
    let asks = vec![Order::new(101.0, 1.0).unwrap()];

    bot.update_order_books(exchange, bids.clone(), asks.clone());

    let order_book = bot.get_order_book(exchange);
    assert!(order_book.is_some());
    let order_book = order_book.unwrap();
    assert_eq!(order_book.0, bids);
    assert_eq!(order_book.1, asks);
}

#[test]
fn test_check_arbitrage_with_order_books() {
    let mut bot = ArbitrageBot::new();
    
    let exchange1 = "Exchange1";
    let exchange2 = "Exchange2";
    let exchange1_bids = vec![Order::new(100.0, 1.0).unwrap()];
    let exchange1_asks = vec![Order::new(101.0, 1.0).unwrap()];
    let exchange2_bids = vec![Order::new(102.0, 1.0).unwrap()];
    let exchange2_asks = vec![Order::new(99.0, 1.0).unwrap()];

    bot.update_order_books(exchange1, exchange1_bids, exchange1_asks);
    bot.update_order_books(exchange2, exchange2_bids, exchange2_asks);

    let result = bot.check_arbitrage(exchange1, exchange2);
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[test]
fn test_process_order_books_and_check_arbitrage() {
    let mut bot = ArbitrageBot::new();
    
    let exchange1_bids = vec![Order::new(100.0, 1.0).unwrap()];
    let exchange1_asks = vec![Order::new(101.0, 1.0).unwrap()];
    let exchange2_bids = vec![Order::new(102.0, 1.0).unwrap()];
    let exchange2_asks = vec![Order::new(99.0, 1.0).unwrap()];

    bot.process_order_books(
        "Exchange1",
        "Exchange2",
        exchange1_bids,
        exchange1_asks,
        exchange2_bids,
        exchange2_asks
    );

    let result = bot.check_arbitrage("Exchange1", "Exchange2");
    println!("Check arbitrage result: {:?}", result);
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}


#[test]
fn test_run_method() {
    let mut bot = ArbitrageBot::new();
    
    let exchange1_bids = vec![Order::new(100.0, 1.0).unwrap()];
    let exchange1_asks = vec![Order::new(101.0, 1.0).unwrap()];
    let exchange2_bids = vec![Order::new(102.0, 1.0).unwrap()];
    let exchange2_asks = vec![Order::new(99.0, 1.0).unwrap()];

    bot.process_order_books(
        "Exchange1",
        "Exchange2",
        exchange1_bids,
        exchange1_asks,
        exchange2_bids,
        exchange2_asks
    );

    bot.run("Exchange1", "Exchange2", Duration::from_secs(1));
}
