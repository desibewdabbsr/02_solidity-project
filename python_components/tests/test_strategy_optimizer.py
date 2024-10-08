import unittest
import numpy as np
from python_components.src.strategy_optimizer import StrategyOptimizer
from python_components.logger import logger

class TestStrategyOptimizer(unittest.TestCase):
    def setUp(self):
        self.optimizer = StrategyOptimizer()
        logger.info("Setting up TestStrategyOptimizer")

    def tearDown(self):
        logger.info("Tearing down TestStrategyOptimizer")

    def test_optimize_allocation(self):
        expected_returns = (0.05, 0.03, 0.04)
        risks = (0.1, 0.05, 0.08)
        allocation = self.optimizer.optimize_allocation(expected_returns, risks)
        
        self.assertEqual(len(allocation), len(expected_returns))
        self.assertAlmostEqual(sum(allocation), 1.0, places=6)
        self.assertTrue(all(0 <= x <= self.optimizer.max_allocation for x in allocation))
        logger.info("Optimize allocation test passed")

    def test_objective_function(self):
        weights = np.array([0.3, 0.3, 0.4])
        expected_returns = (0.05, 0.03, 0.04)
        risks = (0.1, 0.05, 0.08)
        
        result = self.optimizer._objective_function(weights, expected_returns, risks)
        self.assertIsInstance(result, float)
        logger.info("Objective function test passed")

    def test_max_allocation_constraint(self):
        expected_returns = (0.1, 0.05)
        risks = (0.2, 0.1)
        allocation = self.optimizer.optimize_allocation(expected_returns, risks)
        
        self.assertTrue(all(x <= self.optimizer.max_allocation for x in allocation))
        logger.info("Max allocation constraint test passed")

if __name__ == '__main__':
    unittest.main()
