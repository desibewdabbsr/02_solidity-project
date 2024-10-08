# arbitrage_bot.py

import logging
from typing import Dict, Any, List
from python_components.src.data_fetcher import DataFetcher
from python_components.src.price_predictor import PricePredictor
from python_components.src.risk_assessor import RiskAssessor
from python_components.src.strategy_optimizer import StrategyOptimizer

logger = logging.getLogger(__name__)

class ArbitrageBot:
    def __init__(self):
        self.data_fetcher = DataFetcher()
        self.price_predictor = PricePredictor()
        self.risk_assessor = RiskAssessor()
        self.strategy_optimizer = StrategyOptimizer()
        self.exchanges = ["Uniswap", "SushiSwap"]
        logger.info("ArbitrageBot initialized with exchanges: %s", self.exchanges)

    def run(self, token_pair: str):
        logger.info(f"Running arbitrage for {token_pair}")
        try:
            market_data = self._fetch_market_data(token_pair)
            predictions = self._predict_prices(token_pair)
            if predictions is None:
                logger.warning("No valid predictions. Skipping further processing.")
                return
            risks = self._assess_risks(market_data, predictions)
            allocation = self._optimize_strategy(predictions, risks)
            if allocation is not None:
                self._execute_trades(token_pair, allocation)
            else:
                logger.warning("No optimal allocation found. Skipping trade execution.")
        except Exception as e:
            logger.exception(f"Error during arbitrage run: {str(e)}")
            raise

    def _fetch_market_data(self, token_pair: str) -> Dict[str, Any]:
        logger.info(f"Fetching market data for {token_pair}")
        return {exchange: self.data_fetcher.fetch_market_data(exchange, token_pair) for exchange in self.exchanges}

    def _predict_prices(self, token_pair: str) -> Dict[str, float]:
        logger.info(f"Predicting prices for {token_pair}")
        historical_data = {exchange: self.data_fetcher.fetch_historical_data(exchange, token_pair, '1h') for exchange in self.exchanges}
        predictions = {exchange: self.price_predictor.predict(data.get('prices', [])) for exchange, data in historical_data.items()}
        if all(prediction == 0.0 for prediction in predictions.values()):
            logger.warning("All price predictions are 0.0. Returning None.")
            return None
        return predictions

    def _assess_risks(self, market_data: Dict[str, Any], predictions: Dict[str, float]) -> Dict[str, float]:
        logger.info("Assessing risks")
        return {exchange: self.risk_assessor.assess_risk(market_data[exchange], predictions[exchange]) 
                for exchange in self.exchanges}

    def _optimize_strategy(self, predictions: Dict[str, float], risks: Dict[str, float]) -> List[float]:
        logger.info("Optimizing strategy")
        expected_returns = [predictions[exchange] for exchange in self.exchanges]
        risk_values = [risks[exchange] for exchange in self.exchanges]
        return self.strategy_optimizer.optimize_allocation(expected_returns, risk_values)

    def _execute_trades(self, token_pair: str, allocation: List[float]):
        logger.info(f"Executing trades for {token_pair}")
        for exchange, alloc in zip(self.exchanges, allocation):
            if alloc > 0:
                logger.info(f"Executing trade on {exchange} for {token_pair} with allocation {alloc}")
                # Implement actual trade execution logic here
