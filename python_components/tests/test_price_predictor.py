import unittest
import numpy as np
from python_components.src.price_predictor import PricePredictor
from python_components.logger import logger

class TestPricePredictor(unittest.TestCase):
    def setUp(self):
        self.predictor = PricePredictor()
        logger.info("Setting up TestPricePredictor")

    def tearDown(self):
        logger.info("Tearing down TestPricePredictor")

    def test_model_initialization(self):
        self.assertIsNotNone(self.predictor.model)
        logger.info("Model initialization test passed")

    def test_data_preparation(self):
        data = list(range(100))
        X, y = self.predictor._prepare_data(data)
        self.assertEqual(X.shape, (40, 60, 1))
        self.assertEqual(y.shape, (40,))
        logger.info("Data preparation test passed")

    def test_train_and_predict(self):
        historical_data = np.sin(np.linspace(0, 10, 1000)).tolist()
        self.predictor.train(historical_data)
        
        recent_data = historical_data[-60:]
        prediction = self.predictor.predict(recent_data)
        
        self.assertIsInstance(prediction, float)
        self.assertTrue(-1 <= prediction <= 1)
        logger.info("Train and predict test passed")

    def test_prediction_shape(self):
        recent_data = np.random.rand(60).tolist()
        prediction = self.predictor.predict(recent_data)
        self.assertIsInstance(prediction, float)
        logger.info("Prediction shape test passed")

if __name__ == '__main__':
    unittest.main()

