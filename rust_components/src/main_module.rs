use std::collections::HashMap;
use crate::order::Order;
use crate::data_processor::DataProcessor;
use crate::arbitrage_engine::ArbitrageEngine;
use std::time::{Duration, Instant};
use std::thread;

pub struct ArbitrageBot {
    data_processor: DataProcessor,
    arbitrage_engine: ArbitrageEngine,
    order_books: HashMap<String, (Vec<Order>, Vec<Order>)>,
}

impl ArbitrageBot {
    pub fn new() -> Self {
        ArbitrageBot {
            data_processor: DataProcessor::new(),
            arbitrage_engine: ArbitrageEngine::new(),
            order_books: HashMap::new(),
        }
    }

    pub fn process_order_books(
        &mut self,
        exchange1: &str,
        exchange2: &str,
        exchange1_bids: Vec<Order>,
        exchange1_asks: Vec<Order>,
        exchange2_bids: Vec<Order>,
        exchange2_asks: Vec<Order>,
    ) {
        println!("Processing order books for {} and {}", exchange1, exchange2);
        self.update_order_books(exchange1, exchange1_bids, exchange1_asks);
        self.update_order_books(exchange2, exchange2_bids, exchange2_asks);
        println!("Order books processed");
    }
    

    pub fn update_order_books(&mut self, exchange: &str, bids: Vec<Order>, asks: Vec<Order>) {
        self.order_books.insert(exchange.to_string(), (bids.clone(), asks.clone()));
        self.data_processor.process_order_book_update(exchange, bids, true);
        self.data_processor.process_order_book_update(exchange, asks, false);
    }
    

    pub fn get_order_book(&self, exchange: &str) -> Option<&(Vec<Order>, Vec<Order>)> {
        self.order_books.get(exchange)
    }


    pub fn check_arbitrage(&self, exchange1: &str, exchange2: &str) -> Result<Option<f64>, String> {
        println!("Checking arbitrage between {} and {}", exchange1, exchange2);
        let order_book1 = self.get_order_book(exchange1)
            .ok_or_else(|| format!("OrderBookNotFound for {}", exchange1))?;
        let order_book2 = self.get_order_book(exchange2)
            .ok_or_else(|| format!("OrderBookNotFound for {}", exchange2))?;
    
        println!("Order book 1: {:?}", order_book1);
        println!("Order book 2: {:?}", order_book2);
    
        let result = self.arbitrage_engine.calculate_arbitrage(
            &order_book1.0,
            &order_book1.1,
            &order_book2.0,
            &order_book2.1
        );
        println!("Arbitrage result: {:?}", result);
        result
    }
    

    



    

    pub fn run(&mut self, exchange1: &str, exchange2: &str, duration: Duration) {
        println!("Running arbitrage check between {} and {} for {} seconds", exchange1, exchange2, duration.as_secs());
        
        let start_time = Instant::now();
        while start_time.elapsed() < duration {
            match self.check_arbitrage(exchange1, exchange2) {
                Ok(Some(profit)) => println!("Arbitrage opportunity found! Potential profit: {}", profit),
                Ok(None) => println!("No arbitrage opportunity at this time."),
                Err(e) => println!("Error checking arbitrage: {}", e),
            }
            thread::sleep(Duration::from_secs(1));
        }
        
        println!("Arbitrage check completed.");
    }
}
