import requests
from typing import Dict, Any
from python_components.logger import logger
import os
from dotenv import load_dotenv


# For RPC_URL Add this line to specify the correct path to your .env file
dotenv_path = '/Users/macbook/Desktop/solidity-project/advanced-arbitrage-bot/.env'


class DataFetcher:
    def __init__(self):
        load_dotenv()
        self.base_url = os.getenv('SEPOLIA_RPC_URL')
        logger.info(f"DataFetcher initialized with base URL: {self.base_url}")

    def fetch_market_data(self, dex: str, token_pair: str) -> Dict[str, Any]:
        logger.info(f"Fetching market data for {token_pair} on {dex}")
        try:
            response = requests.get(f"{self.base_url}/market_data/{dex}/{token_pair}", timeout=10)
            response.raise_for_status()
            data = response.json()
            logger.info(f"Successfully fetched market data for {token_pair} on {dex}")
            return data
        except requests.RequestException as e:
            logger.error(f"Error fetching market data: {str(e)}")
            raise

    def fetch_historical_data(self, dex: str, token_pair: str, timeframe: str) -> Dict[str, Any]:
        logger.info(f"Fetching historical data for {token_pair} on {dex} with timeframe {timeframe}")
        try:
            response = requests.get(f"{self.base_url}/historical_data/{dex}/{token_pair}/{timeframe}", timeout=10)
            response.raise_for_status()
            data = response.json()
            logger.info(f"Successfully fetched historical data for {token_pair} on {dex}")
            return data
        except requests.RequestException as e:
            logger.error(f"Error fetching historical data: {str(e)}")
            raise
