use log::{debug, error};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct Order {
    pub price: f64,
    pub amount: f64,
}

#[derive(Error, Debug)]
pub enum OrderError {
    #[error("Invalid price: {0}")]
    InvalidPrice(f64),
    #[error("Invalid amount: {0}")]
    InvalidAmount(f64),
}

impl Order {
    pub fn new(price: f64, amount: f64) -> Result<Self, OrderError> {
        if price <= 0.0 {
            error!("Attempted to create order with invalid price: {}", price);
            return Err(OrderError::InvalidPrice(price));
        }
        if amount <= 0.0 {
            error!("Attempted to create order with invalid amount: {}", amount);
            return Err(OrderError::InvalidAmount(amount));
        }
        debug!("Created new order: price={}, amount={}", price, amount);
        Ok(Order { price, amount })
    }
}
