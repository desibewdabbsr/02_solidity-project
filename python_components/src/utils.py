# python_components/src/utils.py

import numpy as np
from typing import List
from python_components.logger import logger

def format_profit(profit: float) -> str:
    return f"${profit:.2f}"

def calculate_percentage(value: float, total: float) -> float:
    return (value / total) * 100 if total != 0 else 0

def calculate_moving_average(data: List[float], window: int) -> List[float]:
    return list(np.convolve(data, np.ones(window), 'valid') / window)

def calculate_volatility(prices: List[float]) -> float:
    returns = np.diff(prices) / prices[:-1]
    return np.std(returns)

def calculate_sharpe_ratio(returns: List[float], risk_free_rate: float = 0.02) -> float:
    excess_returns = np.array(returns) - risk_free_rate
    mean_excess_return = np.mean(excess_returns)
    std_dev = np.std(excess_returns, ddof=1)
    if std_dev == 0:
        return 0
    sharpe_ratio = mean_excess_return / std_dev
    logger.info(f"Calculated Sharpe ratio: {sharpe_ratio}")
    return sharpe_ratio

def normalize_data(data: List[float]) -> List[float]:
    min_val, max_val = min(data), max(data)
    return [(x - min_val) / (max_val - min_val) if max_val != min_val else 0 for x in data]
