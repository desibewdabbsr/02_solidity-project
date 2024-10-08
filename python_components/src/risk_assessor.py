import numpy as np
from typing import Dict, Any
from python_components.logger import logger

class RiskAssessor:
    def __init__(self, risk_threshold: float = 0.05):
        self.risk_threshold = risk_threshold
        logger.info(f"RiskAssessor initialized with risk threshold: {self.risk_threshold}")

    def assess_risk(self, market_data: Dict[str, Any], predicted_price: float) -> np.ndarray:
        historical_prices = market_data.get('historical_prices', [])
        volatility = self._calculate_volatility(historical_prices) if historical_prices else 0.0
        liquidity = market_data.get('liquidity', 1)
        current_price = market_data.get('current_price', predicted_price)
        price_difference = abs(current_price - predicted_price) / current_price if current_price != 0 else 0

        risk_score = (volatility * 0.4) + (1/liquidity * 0.3) + (price_difference * 0.3)
        logger.info(f"Calculated risk score: {risk_score}")
        return np.array([risk_score])

        
    def is_trade_safe(self, risk_score: float) -> bool:
        is_safe = risk_score < self.risk_threshold
        logger.info(f"Trade safety assessment: {'Safe' if is_safe else 'Unsafe'}")
        return is_safe

    def _calculate_volatility(self, historical_prices: list) -> float:
        returns = np.diff(historical_prices) / historical_prices[:-1]
        volatility = np.std(returns)
        logger.debug(f"Calculated volatility: {volatility}")
        return volatility
