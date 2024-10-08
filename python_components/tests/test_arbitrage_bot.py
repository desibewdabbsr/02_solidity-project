# test_arbitrage_bot.py

import unittest
from unittest.mock import patch, MagicMock
import numpy as np
import logging
from python_components.src.arbitrage_bot import ArbitrageBot

logger = logging.getLogger(__name__)

class TestArbitrageBot(unittest.TestCase):
    def setUp(self):
        self.bot = ArbitrageBot()
        logger.info("Setting up TestArbitrageBot")

    def tearDown(self):
            logger.info("Tearing down TestArbitrageBot")
    @patch('python_components.src.arbitrage_bot.DataFetcher')
    @patch('python_components.src.arbitrage_bot.PricePredictor')
    @patch('python_components.src.arbitrage_bot.RiskAssessor')
    @patch('python_components.src.arbitrage_bot.StrategyOptimizer')
    def test_run(self, mock_optimizer, mock_risk, mock_predictor, mock_fetcher):
        mock_market_data = {
            'price': 100,
            'volume': 1000,
            'liquidity': 10000,
            'current_price': 100,
            'historical_prices': [90, 95, 100] * 20
        }
        mock_fetcher_instance = mock_fetcher.return_value
        mock_fetcher_instance.fetch_market_data.return_value = mock_market_data
        mock_fetcher_instance.fetch_historical_data.return_value = {'prices': [90, 95, 100] * 20}
        
        mock_predictor.return_value.predict.return_value = 105.0
        mock_risk.return_value.assess_risk.return_value = np.array([0.1])
        mock_optimizer.return_value.optimize_allocation.return_value = [0.6, 0.4]

        self.bot.run('ETH-USDT')

        mock_fetcher_instance.fetch_market_data.assert_called()
        mock_fetcher_instance.fetch_historical_data.assert_called()
        mock_predictor.return_value.predict.assert_called()
        mock_risk.return_value.assess_risk.assert_called()
        mock_optimizer.return_value.optimize_allocation.assert_called()

        logger.info("Run test passed successfully")

    def test_fetch_market_data(self):
        mock_market_data = {
            'price': 100,
            'volume': 1000,
            'liquidity': 10000,
            'current_price': 100,
            'historical_prices': [90, 95, 100] * 20
        }
        with patch.object(self.bot.data_fetcher, 'fetch_market_data', return_value=mock_market_data):
            result = self.bot._fetch_market_data('ETH-USDT')
            self.assertEqual(len(result), len(self.bot.exchanges))
            self.assertIn('Uniswap', result)
            self.assertIn('SushiSwap', result)
        logger.info("Fetch market data test passed")

    def test_predict_prices(self):
        with patch.object(self.bot.data_fetcher, 'fetch_historical_data', return_value={'prices': [90, 95, 100] * 20}):
            with patch.object(self.bot.price_predictor, 'predict', return_value=105):
                result = self.bot._predict_prices('ETH-USDT')
                self.assertEqual(len(result), len(self.bot.exchanges))
                self.assertIn('Uniswap', result)
                self.assertIn('SushiSwap', result)
        logger.info("Predict prices test passed")

if __name__ == '__main__':
    logging.basicConfig(level=logging.INFO)
    unittest.main()
