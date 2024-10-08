const logger = require('./logger');


function detectArbitrageOpportunity(prices, threshold = 0.02) {
    logger.debug(`Starting arbitrage detection with prices: ${JSON.stringify(prices)}, threshold: ${threshold}`);

    try {
        if (!prices || typeof prices !== 'object' || Object.keys(prices).length < 2) {
            throw new Error('Invalid or insufficient price data');
        }

        if (typeof threshold !== 'number' || threshold <= 0 || threshold >= 1) {
            throw new Error('Invalid threshold value. Must be a number between 0 and 1');
        }

        const validPrices = Object.entries(prices).filter(([, price]) => typeof price === 'number' && !isNaN(price) && price > 0);
        if (validPrices.length < 2) {
            throw new Error('No valid price data');
        }

        const minPrice = Math.min(...validPrices.map(([, price]) => price));
        logger.debug(`Minimum price: ${minPrice}`);

        const opportunities = validPrices
            .map(([dex, price]) => {
                const deviation = (price - minPrice) / minPrice;
                logger.debug(`${dex}: Price = ${price}, Deviation = ${deviation.toFixed(4)}`);
                return [dex, price, deviation];
            })
            .filter(([, , deviation]) => deviation >= threshold);

        if (opportunities.length > 0) {
            const result = opportunities.map(([dex, price]) => [dex, price]);
            logger.info(`Arbitrage opportunities detected: ${JSON.stringify(result)}`);
            return result;
        } else {
            logger.info('No arbitrage opportunities detected');
            return null;
        }
    } catch (error) {
        logger.error(`Error in detectArbitrageOpportunity: ${error.message}`, { stack: error.stack });
        throw error;
    }
}


function validateInput(prices, threshold) {
    if (!prices || typeof prices !== 'object' || Object.keys(prices).length < 2) {
        throw new Error('Invalid or insufficient price data');
    }
    if (typeof threshold !== 'number' || threshold <= 0 || threshold >= 1) {
        throw new Error('Invalid threshold value. Must be a number between 0 and 1');
    }
    logger.debug('Input validation passed');
}

function getWeightedAveragePrice(prices) {
    const validPrices = Object.values(prices).filter(price => typeof price === 'number' && !isNaN(price) && price > 0);
    if (validPrices.length === 0) {
        throw new Error('No valid price data');
    }
    const totalPrice = validPrices.reduce((sum, price) => sum + price, 0);
    return totalPrice / validPrices.length;
}

module.exports = detectArbitrageOpportunity;
