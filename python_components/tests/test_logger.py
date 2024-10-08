# python_components/tests/test_logger.py

import unittest
import os
import logging
import sys

# Add the parent directory to sys.path to import the logger module
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))

from logger import setup_logger, logger


class TestLogger(unittest.TestCase):
    def setUp(self):
        self.log_dir = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', 'logs')
        self.log_file = os.path.join(self.log_dir, 'arbitrage_bot.log')
        if os.path.exists(self.log_file):
            os.remove(self.log_file)
        # Ensure the logger is set up for each test
        self.logger = setup_logger()

    def test_file_handler(self):
        self.logger.debug("Debug message")
        self.logger.info("Info message")
        self.logger.warning("Warning message")
        self.logger.error("Error message")

        # Wait for a short time to ensure file writing is complete
        import time
        time.sleep(0.1)

        self.assertTrue(os.path.exists(self.log_file), f"Log file was not created at {self.log_file}")

        with open(self.log_file, 'r') as f:
            log_contents = f.read()

        self.assertIn("DEBUG - Debug message", log_contents)
        self.assertIn("INFO - Info message", log_contents)
        self.assertIn("WARNING - Warning message", log_contents)
        self.assertIn("ERROR - Error message", log_contents)

    def test_log_formatting(self):
        self.logger.info("Test formatting")
        
        # Wait for a short time to ensure file writing is complete
        import time
        time.sleep(0.1)

        self.assertTrue(os.path.exists(self.log_file), f"Log file was not created at {self.log_file}")

        with open(self.log_file, 'r') as f:
            last_line = f.readlines()[-1]

        self.assertRegex(last_line, r'\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2},\d{3} - arbitrage_bot - INFO - Test formatting')


if __name__ == '__main__':
    unittest.main()
