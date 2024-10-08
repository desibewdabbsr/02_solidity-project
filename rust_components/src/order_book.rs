use crate::order::Order;
use log::{debug, error, info};
use thiserror::Error;
use std::collections::BTreeMap;
use ordered_float::OrderedFloat;

#[derive(Debug)]
pub struct OrderBook {
    bids: BTreeMap<OrderedFloat<f64>, f64>,
    asks: BTreeMap<OrderedFloat<f64>, f64>,
}


#[derive(Error, Debug, PartialEq)]
pub enum OrderBookError {
    #[error("Invalid order type")]
    InvalidOrderType,
    #[error("Order price already exists")]
    DuplicatePrice,
}

impl OrderBook {
    pub fn new() -> Self {
        info!("Creating new OrderBook");
        OrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn from_orders(bids: Vec<Order>, asks: Vec<Order>) -> Self {
        let mut book = OrderBook::new();
        for bid in bids {
            book.add_order(&bid, true).unwrap();
        }
        for ask in asks {
            book.add_order(&ask, false).unwrap();
        }
        book
    }

    pub fn add_order(&mut self, order: &Order, is_bid: bool) -> Result<(), OrderBookError> {
        let book = if is_bid { &mut self.bids } else { &mut self.asks };

        if book.contains_key(&OrderedFloat(order.price)) {
            error!("Attempted to add order with duplicate price: {}", order.price);
            return Err(OrderBookError::DuplicatePrice);
        }

        book.insert(OrderedFloat(order.price), order.amount);
        debug!("Added {} order: price={}, amount={}", if is_bid { "bid" } else { "ask" }, order.price, order.amount);
        Ok(())
    }
    pub fn get_best_bid(&self) -> Option<(f64, f64)> {
        self.bids.iter().next_back().map(|(price, amount)| (price.into_inner(), *amount))
    }

    pub fn get_best_ask(&self) -> Option<(f64, f64)> {
        self.asks.iter().next().map(|(price, amount)| (price.into_inner(), *amount))
    }

    pub fn get_all_bids(&self) -> Vec<Order> {
        self.bids.iter().map(|(price, amount)| Order::new(price.into_inner(), *amount).unwrap()).collect()
    }

    pub fn get_all_asks(&self) -> Vec<Order> {
        self.asks.iter().map(|(price, amount)| Order::new(price.into_inner(), *amount).unwrap()).collect()
    }


}
