pub mod order;
pub mod order_book;
pub mod arbitrage_calculator;
pub mod arbitrage_engine;
pub mod errors;
pub mod data_processor;
pub mod main_module;


#[no_mangle]
pub extern "C" fn check_arbitrage(exchange1: *const c_char, exchange2: *const c_char) -> bool {
    // Your Rust implementation here
}
