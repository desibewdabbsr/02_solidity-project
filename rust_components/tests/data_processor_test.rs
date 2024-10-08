use rust_components::data_processor::DataProcessor;
use rust_components::order::Order;

#[test]
fn test_process_order_book_update() {
    let mut processor = DataProcessor::new();
    let orders = vec![
        Order::new(100.0, 1.0).unwrap(),
        Order::new(101.0, 2.0).unwrap(),
    ];
    processor.process_order_book_update("Exchange1", orders, true);

    let order_book = processor.get_order_book("Exchange1").unwrap();
    assert_eq!(order_book.get_best_bid(), Some((101.0, 2.0)));
}

#[test]
fn test_calculate_spread() {
    let mut processor = DataProcessor::new();
    let bid_orders = vec![Order::new(100.0, 1.0).unwrap()];
    let ask_orders = vec![Order::new(102.0, 1.0).unwrap()];
    
    processor.process_order_book_update("Exchange1", bid_orders, true);
    processor.process_order_book_update("Exchange1", ask_orders, false);

    assert_eq!(processor.calculate_spread("Exchange1"), Some(2.0));
}

#[test]
fn test_multiple_exchanges() {
    let mut processor = DataProcessor::new();
    let orders1 = vec![Order::new(100.0, 1.0).unwrap()];
    let orders2 = vec![Order::new(101.0, 1.0).unwrap()];
    
    processor.process_order_book_update("Exchange1", orders1, true);
    processor.process_order_book_update("Exchange2", orders2, true);

    assert_eq!(processor.get_order_book("Exchange1").unwrap().get_best_bid(), Some((100.0, 1.0)));
    assert_eq!(processor.get_order_book("Exchange2").unwrap().get_best_bid(), Some((101.0, 1.0)));
}
