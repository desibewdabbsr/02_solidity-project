use crate::order_book::OrderBook;
use crate::order::Order;
use std::collections::HashMap;

pub struct DataProcessor {
    exchange_data: HashMap<String, OrderBook>,
}

impl DataProcessor {
    pub fn new() -> Self {
        DataProcessor {
            exchange_data: HashMap::new(),
        }
    }

    pub fn process_order_book_update(&mut self, exchange: &str, orders: Vec<Order>, is_bid: bool) {
        let order_book = self.exchange_data.entry(exchange.to_string()).or_insert(OrderBook::new());
        for order in orders {
            if let Err(e) = order_book.add_order(&order, is_bid) {
                eprintln!("Error adding order to {}: {:?}", exchange, e);
            }
        }
    }

    pub fn get_order_book(&self, exchange: &str) -> Option<&OrderBook> {
        self.exchange_data.get(exchange)
    }

    pub fn calculate_spread(&self, exchange: &str) -> Option<f64> {
        if let Some(order_book) = self.get_order_book(exchange) {
            if let (Some(best_bid), Some(best_ask)) = (order_book.get_best_bid(), order_book.get_best_ask()) {
                return Some(best_ask.0 - best_bid.0);
            }
        }
        None
    }
}
