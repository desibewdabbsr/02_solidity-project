use rust_components::arbitrage_calculator::ArbitrageCalculator;

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_arbitrage_opportunity() {
        init();
        let arbitrage = ArbitrageCalculator::calculate_arbitrage(Some(100.5), Some(100.0));
        assert_eq!(arbitrage, Some(0.5));
    }

    #[test]
    fn test_no_arbitrage_opportunity() {
        init();
        let arbitrage = ArbitrageCalculator::calculate_arbitrage(Some(100.0), Some(100.5));
        assert_eq!(arbitrage, None);
    }

    #[test]
    fn test_missing_bid() {
        init();
        let arbitrage = ArbitrageCalculator::calculate_arbitrage(None, Some(100.0));
        assert_eq!(arbitrage, None);
    }

    #[test]
    fn test_missing_ask() {
        init();
        let arbitrage = ArbitrageCalculator::calculate_arbitrage(Some(100.0), None);
        assert_eq!(arbitrage, None);
    }
}
