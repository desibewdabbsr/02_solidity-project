import tensorflow as tf
import numpy as np
from typing import List
from python_components.logger import logger

class PricePredictor:
    def __init__(self):
        self.model = self._build_model()
        logger.info("PricePredictor initialized")

    def _build_model(self) -> tf.keras.Model:
        model = tf.keras.Sequential([
            tf.keras.layers.LSTM(50, return_sequences=True, input_shape=(60, 1)),
            tf.keras.layers.LSTM(50, return_sequences=False),
            tf.keras.layers.Dense(25),
            tf.keras.layers.Dense(1)
        ])
        model.compile(optimizer='adam', loss='mean_squared_error')
        logger.info("LSTM model built and compiled")
        return model

    def train(self, historical_data: List[float]):
        X, y = self._prepare_data(historical_data)
        self.model.fit(X, y, epochs=10, batch_size=32, verbose=0)
        logger.info("Model training completed")

    def predict(self, recent_data: List[float]) -> float:
        if len(recent_data) < 60:
            logger.warning(f"Insufficient data for prediction. Expected at least 60 data points, got {len(recent_data)}")
            return 0.0  # or some default value
        X = np.array(recent_data[-60:]).reshape(1, 60, 1)
        prediction = self.model.predict(X, verbose=0)
        logger.info(f"Price prediction: {prediction[0][0]}")
        return float(prediction[0][0])

    def _prepare_data(self, data: List[float]):
        X, y = [], []
        for i in range(60, len(data)):
            X.append(data[i-60:i])
            y.append(data[i])
        logger.debug(f"Prepared {len(X)} data points for training")
        return np.array(X).reshape(-1, 60, 1), np.array(y)

