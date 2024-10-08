const ethers = require('ethers');
const IUniswapV2Pair = require('@uniswap/v2-core/build/IUniswapV2Pair.json');
const logger = require('./utils/logger');
const detectArbitrageOpportunity = require('./utils/detectArbitrageOpportunity');

class PriceMonitor {
    constructor(provider, dexAddresses) {
        this.provider = new ethers.providers.JsonRpcProvider(provider);
        this.dexAddresses = dexAddresses;
        this.pairContracts = {};
        this.lastPrices = {};
    }

    async initializePairs() {
        try {
            for (const [dex, pairAddress] of Object.entries(this.dexAddresses)) {
                this.pairContracts[dex] = new ethers.Contract(pairAddress, IUniswapV2Pair.abi, this.provider);
            }
            logger.info('Pair contracts initialized successfully');
        } catch (error) {
            logger.error('Error initializing pair contracts:', error);
            throw error;
        }
    }

    async getPrice(dex) {
        try {
            const pairContract = this.pairContracts[dex];
            const reserves = await pairContract.getReserves();
            if (!reserves || !Array.isArray(reserves) || reserves.length < 2) {
                logger.warn(`Invalid reserves data for ${dex}`);
                return 0;
            }
            if (!reserves[0] || !reserves[1] || typeof reserves[0].isZero !== 'function' || typeof reserves[1].isZero !== 'function') {
                logger.warn(`Invalid reserve values for ${dex}`);
                return 0;
            }
            if (reserves[0].isZero() || reserves[1].isZero()) {
                logger.warn(`Zero reserves detected for ${dex}`);
                return 0;
            }
            const price = reserves[0].div(reserves[1]).toNumber();
            logger.debug(`Price for ${dex}: ${price}`);
            return price;
        } catch (error) {
            logger.error(`Error getting price for ${dex}:`, error);
            return 0;
        }
    }

    async monitorPrices(callback, interval = 10000) {
        const monitor = async () => {
            const prices = {};
            for (const dex of Object.keys(this.dexAddresses)) {
                prices[dex] = await this.getPrice(dex);
            }
            callback(prices);
        };
    
        await monitor(); // Call immediately once
        return setInterval(monitor, interval);
    }

    detectArbitrageOpportunity(prices, threshold = 0.02) {
        return detectArbitrageOpportunity(prices, threshold);
    }
}

module.exports = PriceMonitor;
