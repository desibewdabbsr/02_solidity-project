use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArbitrageEngineError {
    #[error("Best bid not found")]
    BestBidError,
    #[error("Best ask not found")]
    BestAskError,
    #[error("Order book not found")]
    OrderBookNotFound,
}


// use std::fmt;
// use thiserror::Error;

// #[derive(Error, Debug)]
// pub enum ArbitrageEngineError {
//     #[error("Best bid not found")]
//     BestBidError,
//     #[error("Best ask not found")]
//     BestAskError,
//     #[error("Order book not found")]
//     OrderBookNotFound,
// }

// impl fmt::Display for ArbitrageEngineError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             ArbitrageEngineError::BestBidError => write!(f, "Failed to get the best bid"),
//             ArbitrageEngineError::BestAskError => write!(f, "Failed to get the best ask"),
//             ArbitrageEngineError::OrderBookNotFound => write!(f, "Order book not found"),
//         }
//     }
// }

// impl std::error::Error for ArbitrageEngineError {}
