import unittest
import numpy as np
from python_components.src.risk_assessor import RiskAssessor
from python_components.logger import logger

class TestRiskAssessor(unittest.TestCase):
    def setUp(self):
        self.risk_assessor = RiskAssessor()
        logger.info("Setting up TestRiskAssessor")

    def tearDown(self):
        logger.info("Tearing down TestRiskAssessor")

    def test_assess_risk(self):
        market_data = {
            'historical_prices': [100, 101, 99, 102, 98],
            'liquidity': 1000000,
            'current_price': 100
        }
        predicted_price = 105
        risk_score = self.risk_assessor.assess_risk(market_data, predicted_price)
        self.assertIsInstance(risk_score, float)
        self.assertTrue(0 <= risk_score <= 1)
        logger.info("Risk assessment test passed")

    def test_is_trade_safe(self):
        safe_score = 0.03
        unsafe_score = 0.07
        self.assertTrue(self.risk_assessor.is_trade_safe(safe_score))
        self.assertFalse(self.risk_assessor.is_trade_safe(unsafe_score))
        logger.info("Trade safety test passed")

    def test_calculate_volatility(self):
        historical_prices = [100, 101, 99, 102, 98]
        volatility = self.risk_assessor._calculate_volatility(historical_prices)
        self.assertIsInstance(volatility, float)
        self.assertTrue(volatility > 0)
        logger.info("Volatility calculation test passed")

if __name__ == '__main__':
    unittest.main()
