const { BigNumber } = require('ethers');
const ArbitrageFinder = require('../src/arbitrageFinderAlgorithm');

describe('ArbitrageFinder', () => {
    let arbitrageFinder;
    let mockPriceMonitor;

    beforeEach(() => {
        mockPriceMonitor = {
            monitorPrices: jest.fn(),
        };
        arbitrageFinder = new ArbitrageFinder(mockPriceMonitor);
    });

    test('findArbitrageOpportunities returns correct opportunities', () => {
        const prices = {
            uniswap: 100,
            sushiswap: 102,
            pancakeswap: 101
        };
        const opportunities = arbitrageFinder.findArbitrageOpportunities(prices);
        expect(opportunities).toEqual([
            { buyDex: 'uniswap', sellDex: 'sushiswap', profit: 0.02 },
            { buyDex: 'uniswap', sellDex: 'pancakeswap', profit: 0.01 },
            { buyDex: 'pancakeswap', sellDex: 'sushiswap', profit: 0.009900990099009901 }
        ]);
    });
    

    test('calculateProfit returns correct profit', () => {
        const profit = arbitrageFinder.calculateProfit(100, 102);
        expect(profit).toBeCloseTo(0.02, 5);
    });

    test('monitorForArbitrageOpportunities calls callback with opportunities', async () => {
        const callback = jest.fn();
        const prices = {
            uniswap: 100,
            sushiswap: 102
        };
        mockPriceMonitor.monitorPrices.mockImplementation(cb => cb(prices));

        await arbitrageFinder.monitorForArbitrageOpportunities(callback);

        expect(callback).toHaveBeenCalledWith([
            { buyDex: 'uniswap', sellDex: 'sushiswap', profit: 0.02 }
        ]);
    });

    test('estimateOptimalTradeAmount returns correct estimation', () => {
        const estimation = arbitrageFinder.estimateOptimalTradeAmount(100, 102, '1000000000000000000');
        expect(estimation.buyAmount.toString()).toBe('1000000000000000000');
        expect(estimation.sellAmount.toString()).toBe('1020000000000000000');
        expect(estimation.profit.toString()).toBe('20000000000000000');
    });
});
