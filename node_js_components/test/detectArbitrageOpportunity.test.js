const detectArbitrageOpportunity = require('../src/utils/detectArbitrageOpportunity');
const logger = require('../src/utils/logger');

jest.mock('../src/utils/logger');

describe('detectArbitrageOpportunity', () => {
    beforeEach(() => {
        jest.clearAllMocks();
    });

    test('returns opportunities when they exist', () => {
        const prices = { uniswap: 100, sushiswap: 103 };
        const opportunities = detectArbitrageOpportunity(prices, 0.02);
        expect(opportunities).toEqual([['sushiswap', 103]]);
        expect(logger.info).toHaveBeenCalledWith(expect.stringContaining('Arbitrage opportunities detected'));
    });

    test('handles threshold at exactly the deviation', () => {
        const prices = { uniswap: 100, sushiswap: 102 };
        const opportunities = detectArbitrageOpportunity(prices, 0.02);
        expect(opportunities).toEqual([['sushiswap', 102]]);
        expect(logger.info).toHaveBeenCalledWith(expect.stringContaining('Arbitrage opportunities detected'));
    });

    test('returns null when no opportunities exist', () => {
        const prices = { uniswap: 100, sushiswap: 101 };
        const opportunities = detectArbitrageOpportunity(prices, 0.02);
        expect(opportunities).toBeNull();
        expect(logger.info).toHaveBeenCalledWith('No arbitrage opportunities detected');
    });

    test('throws error for invalid input', () => {
        expect(() => detectArbitrageOpportunity({}, 0.02)).toThrow('Invalid or insufficient price data');
        expect(() => detectArbitrageOpportunity({ uniswap: 100 }, 0.02)).toThrow('Invalid or insufficient price data');
        expect(() => detectArbitrageOpportunity({ uniswap: 100, sushiswap: 'invalid' }, 0.02)).toThrow('No valid price data');
        expect(() => detectArbitrageOpportunity({ uniswap: 100, sushiswap: 101 }, 1.5)).toThrow('Invalid threshold value');
        expect(logger.error).toHaveBeenCalledWith(expect.stringContaining('Error in detectArbitrageOpportunity'), expect.any(Object));
    });
});
