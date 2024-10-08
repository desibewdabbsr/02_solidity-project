import unittest
from unittest.mock import patch, MagicMock
from io import StringIO
import sys
import os

# Add the project root to the Python path
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), '..', '..')))

from python_components.src.main import main

class TestMain(unittest.TestCase):
    @patch('python_components.src.main.ArbitrageBot')
    @patch('python_components.src.main.time.sleep')
    def test_main_loop(self, mock_sleep, mock_arbitrage_bot):
        # Set up mock ArbitrageBot
        mock_bot_instance = MagicMock()
        mock_arbitrage_bot.return_value = mock_bot_instance
        
        # Set up mock check_arbitrage to return True once and False twice
        mock_bot_instance.check_arbitrage.side_effect = [True, False, False]
        
        # Capture stdout
        captured_output = StringIO()
        sys.stdout = captured_output

        # Run main function, but break after 3 iterations
        with patch('builtins.print'):
            with self.assertRaises(StopIteration):
                with patch('python_components.src.main.time.sleep', side_effect=[None, None, StopIteration]):
                    main()

        # Reset stdout
        sys.stdout = sys.__stdout__

        # Assert that ArbitrageBot was initialized
        mock_arbitrage_bot.assert_called_once()

        # Assert that check_arbitrage was called 3 times
        self.assertEqual(mock_bot_instance.check_arbitrage.call_count, 3)

        # Assert that execute_arbitrage was called once
        mock_bot_instance.execute_arbitrage.assert_called_once_with("Binance", "Coinbase")

        # Assert that sleep was called 2 times with interval 60
        self.assertEqual(mock_sleep.call_count, 2)
        mock_sleep.assert_called_with(60)

        # Check output
        output = captured_output.getvalue()
        self.assertIn("Starting arbitrage bot...", output)
        self.assertIn("Monitoring exchanges: Binance, Coinbase", output)
        self.assertIn("Checking interval: 60 seconds", output)
        self.assertIn("Checking for arbitrage opportunities...", output)
        self.assertIn("Arbitrage opportunity found!", output)
        self.assertIn("No arbitrage opportunity at this time.", output)

if __name__ == '__main__':
    unittest.main()
