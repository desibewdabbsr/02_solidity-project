use rust_components::order_book::{OrderBook, OrderBookError};
use rust_components::order::Order;

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_order_book_creation() {
        init();
        let order_book = OrderBook::new();
        assert!(order_book.get_best_bid().is_none());
        assert!(order_book.get_best_ask().is_none());
    }

    #[test]
    fn test_add_bid_order() {
        init();
        let mut order_book = OrderBook::new();
        let order = Order::new(100.0, 10.0).unwrap();
        assert!(order_book.add_order(&order, true).is_ok());
        assert_eq!(order_book.get_best_bid(), Some((100.0, 10.0)));
    }

    #[test]
    fn test_add_ask_order() {
        init();
        let mut order_book = OrderBook::new();
        let order = Order::new(100.0, 10.0).unwrap();
        assert!(order_book.add_order(&order, false).is_ok());
        assert_eq!(order_book.get_best_ask(), Some((100.0, 10.0)));
    }

    #[test]
    fn test_duplicate_price() {
        init();
        let mut order_book = OrderBook::new();
        let order1 = Order::new(100.0, 10.0).unwrap();
        let order2 = Order::new(100.0, 20.0).unwrap();
        assert!(order_book.add_order(&order1, true).is_ok());
        assert_eq!(order_book.add_order(&order2, true), Err(OrderBookError::DuplicatePrice));
    }

    #[test]
    fn test_multiple_orders() {
        init();
        let mut order_book = OrderBook::new();
        let bid1 = Order::new(100.0, 10.0).unwrap();
        let bid2 = Order::new(101.0, 5.0).unwrap();
        let ask1 = Order::new(102.0, 8.0).unwrap();
        let ask2 = Order::new(103.0, 12.0).unwrap();

        assert!(order_book.add_order(&bid1, true).is_ok());
        assert!(order_book.add_order(&bid2, true).is_ok());
        assert!(order_book.add_order(&ask1, false).is_ok());
        assert!(order_book.add_order(&ask2, false).is_ok());

        assert_eq!(order_book.get_best_bid(), Some((101.0, 5.0)));
        assert_eq!(order_book.get_best_ask(), Some((102.0, 8.0)));
    }
}
#[test]
fn test_new_order_book() {
    let book = OrderBook::new();
    assert!(book.get_best_bid().is_none());
    assert!(book.get_best_ask().is_none());
}

#[test]
fn test_add_bid_order() {
    let mut book = OrderBook::new();
    let order = Order::new(100.0, 5.0).unwrap();
    assert!(book.add_order(&order, true).is_ok());
    assert_eq!(book.get_best_bid(), Some((100.0, 5.0)));
}

#[test]
fn test_add_ask_order() {
    let mut book = OrderBook::new();
    let order = Order::new(100.0, 5.0).unwrap();
    assert!(book.add_order(&order, false).is_ok());
    assert_eq!(book.get_best_ask(), Some((100.0, 5.0)));
}

#[test]
fn test_duplicate_price() {
    let mut book = OrderBook::new();
    let order1 = Order::new(100.0, 5.0).unwrap();
    let order2 = Order::new(100.0, 3.0).unwrap();
    assert!(book.add_order(&order1, true).is_ok());
    assert_eq!(book.add_order(&order2, true), Err(OrderBookError::DuplicatePrice));
}

#[test]
fn test_multiple_orders() {
    let mut book = OrderBook::new();
    let bid1 = Order::new(100.0, 5.0).unwrap();
    let bid2 = Order::new(99.0, 3.0).unwrap();
    let ask1 = Order::new(101.0, 2.0).unwrap();
    let ask2 = Order::new(102.0, 4.0).unwrap();

    assert!(book.add_order(&bid1, true).is_ok());
    assert!(book.add_order(&bid2, true).is_ok());
    assert!(book.add_order(&ask1, false).is_ok());
    assert!(book.add_order(&ask2, false).is_ok());

    assert_eq!(book.get_best_bid(), Some((100.0, 5.0)));
    assert_eq!(book.get_best_ask(), Some((101.0, 2.0)));
}
