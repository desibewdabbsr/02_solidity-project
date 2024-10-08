import logging
from arbitrage_bot import ArbitrageBot

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(name)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

def main():
    bot = ArbitrageBot()
    token_pair = "ETH-USDT"  # Example token pair

    logger.info("Starting arbitrage bot")
    try:
        bot.run(token_pair)
    except Exception as e:
        logger.error(f"An error occurred: {str(e)}")
    logger.info("Arbitrage bot finished")

if __name__ == "__main__":
    main()
