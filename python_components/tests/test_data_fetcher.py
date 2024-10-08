import unittest
from unittest.mock import patch, Mock
import requests
from python_components.src.data_fetcher import DataFetcher
from python_components.logger import logger

class TestDataFetcher(unittest.TestCase):
    def setUp(self):
        self.fetcher = DataFetcher()
        logger.info("Setting up TestDataFetcher")

    def tearDown(self):
        logger.info("Tearing down TestDataFetcher")

    @patch('python_components.src.data_fetcher.requests.get')
    def test_fetch_market_data_success(self, mock_get):
        mock_response = Mock()
        mock_response.json.return_value = {'price': '100', 'volume': '1000'}
        mock_response.raise_for_status.return_value = None
        mock_get.return_value = mock_response

        result = self.fetcher.fetch_market_data('uniswap', 'ETH-USDT')
        
        self.assertEqual(result, {'price': '100', 'volume': '1000'})
        logger.info("Successfully tested fetch_market_data")

    @patch('python_components.src.data_fetcher.requests.get')
    def test_fetch_market_data_http_error(self, mock_get):
        mock_get.side_effect = requests.RequestException("HTTP Error")

        with self.assertRaises(requests.RequestException):
            self.fetcher.fetch_market_data('uniswap', 'ETH-USDT')
        logger.info("Successfully tested fetch_market_data error handling")

    @patch('python_components.src.data_fetcher.requests.get')
    def test_fetch_historical_data_success(self, mock_get):
        mock_response = Mock()
        mock_response.json.return_value = {'prices': [['100', '1'], ['101', '2']]}
        mock_response.raise_for_status.return_value = None
        mock_get.return_value = mock_response

        result = self.fetcher.fetch_historical_data('uniswap', 'ETH-USDT', '1h')
        
        self.assertEqual(result, {'prices': [['100', '1'], ['101', '2']]})
        logger.info("Successfully tested fetch_historical_data")

    @patch('python_components.src.data_fetcher.requests.get')
    def test_fetch_historical_data_http_error(self, mock_get):
        mock_get.side_effect = requests.RequestException("HTTP Error")

        with self.assertRaises(requests.RequestException):
            self.fetcher.fetch_historical_data('uniswap', 'ETH-USDT', '1h')
        logger.info("Successfully tested fetch_historical_data error handling")

if __name__ == '__main__':
    unittest.main()
