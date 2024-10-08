import numpy as np
from scipy.optimize import minimize
from typing import List
from python_components.logger import logger
from functools import lru_cache

class StrategyOptimizer:
    def __init__(self, max_allocation: float = 0.5):
        self.max_allocation = max_allocation
        logger.info(f"StrategyOptimizer initialized with max allocation: {self.max_allocation}")

    @lru_cache(maxsize=128)
    def optimize_allocation(self, expected_returns: tuple, risks: tuple) -> List[float]:
        num_assets = len(expected_returns)
        constraints = ({'type': 'eq', 'fun': lambda x: np.sum(x) - 1},
                       {'type': 'ineq', 'fun': lambda x: self.max_allocation - x})
        bounds = tuple((0, self.max_allocation) for _ in range(num_assets))

        result = minimize(self._objective_function, 
                          np.array([1/num_assets]*num_assets),
                          args=(expected_returns, risks),
                          method='SLSQP',
                          bounds=bounds,
                          constraints=constraints,
                          options={'maxiter': 100, 'ftol': 1e-6})

        logger.info(f"Optimized allocation: {result.x}")
        return result.x.tolist()

    def _objective_function(self, weights: np.ndarray, expected_returns: tuple, risks: tuple) -> float:
        portfolio_return = np.dot(weights, expected_returns)
        portfolio_risk = np.sqrt(np.dot(weights, np.multiply(risks, weights))) if any(risks) else 0
        sharpe_ratio = portfolio_return / portfolio_risk if portfolio_risk != 0 else 0
        logger.debug(f"Objective function value: {-sharpe_ratio}")
        return -sharpe_ratio
