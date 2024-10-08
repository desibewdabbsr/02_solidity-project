const { BigNumber } = require('ethers');

class ArbitrageFinder {
    constructor(priceMonitor, minProfitThreshold = 0.005) {
        this.priceMonitor = priceMonitor;
        this.minProfitThreshold = minProfitThreshold;
    }

    findArbitrageOpportunities(prices) {
        const opportunities = [];
        const dexes = Object.keys(prices);

        for (let i = 0; i < dexes.length; i++) {
            for (let j = i + 1; j < dexes.length; j++) {
                const dexA = dexes[i];
                const dexB = dexes[j];
                const priceA = prices[dexA];
                const priceB = prices[dexB];

                const profitAB = this.calculateProfit(priceA, priceB);
                const profitBA = this.calculateProfit(priceB, priceA);

                if (profitAB > this.minProfitThreshold) {
                    opportunities.push({
                        buyDex: dexA,
                        sellDex: dexB,
                        profit: profitAB
                    });
                }

                if (profitBA > this.minProfitThreshold) {
                    opportunities.push({
                        buyDex: dexB,
                        sellDex: dexA,
                        profit: profitBA
                    });
                }
            }
        }

        return opportunities;
    }

    calculateProfit(buyPrice, sellPrice) {
        return (sellPrice - buyPrice) / buyPrice;
    }

    async monitorForArbitrageOpportunities(callback) {
        this.priceMonitor.monitorPrices((prices) => {
            const opportunities = this.findArbitrageOpportunities(prices);
            if (opportunities.length > 0) {
                callback(opportunities);
            }
        });
    }

    estimateOptimalTradeAmount(buyPrice, sellPrice, maxTradeAmount) {
        const buyAmount = BigNumber.from(maxTradeAmount);
        const sellPriceBN = BigNumber.from(Math.floor(sellPrice * 1e9)).mul(BigNumber.from(1e9));
        const buyPriceBN = BigNumber.from(Math.floor(buyPrice * 1e9)).mul(BigNumber.from(1e9));
        const sellAmount = buyAmount.mul(sellPriceBN).div(buyPriceBN);
        const profit = sellAmount.sub(buyAmount);
        return { buyAmount, sellAmount, profit };
    }
}

module.exports = ArbitrageFinder;
