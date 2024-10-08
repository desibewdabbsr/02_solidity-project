use rust_components::order::{Order, OrderError};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_creation_valid() {
        let order = Order::new(100.0, 10.0);
        assert!(order.is_ok());
        let order = order.unwrap();
        assert_eq!(order.price, 100.0);
        assert_eq!(order.amount, 10.0);
    }

    #[test]
    fn test_order_creation_invalid_price() {
        let order = Order::new(0.0, 10.0);
        assert!(matches!(order, Err(OrderError::InvalidPrice(0.0))));
    }

    #[test]
    fn test_order_creation_invalid_amount() {
        let order = Order::new(100.0, 0.0);
        assert!(matches!(order, Err(OrderError::InvalidAmount(0.0))));
    }

    #[test]
    fn test_order_creation_negative_price() {
        let order = Order::new(-10.0, 10.0);
        assert!(matches!(order, Err(OrderError::InvalidPrice(-10.0))));
    }

    #[test]
    fn test_order_creation_negative_amount() {
        let order = Order::new(100.0, -5.0);
        assert!(matches!(order, Err(OrderError::InvalidAmount(-5.0))));
    }
}
