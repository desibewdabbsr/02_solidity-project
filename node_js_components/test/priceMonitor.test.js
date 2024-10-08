const ethers = require('ethers');
const PriceMonitor = require('../src/priceMonitor');
const detectArbitrageOpportunity = require('../src/utils/detectArbitrageOpportunity');

jest.mock('ethers');
jest.mock('../src/utils/logger');
jest.mock('../src/utils/detectArbitrageOpportunity');

describe('PriceMonitor', () => {
    let priceMonitor;
    let mockProvider;
    let mockPairContract;

    beforeEach(() => {
        mockProvider = {
            getNetwork: jest.fn().mockResolvedValue({ chainId: 1 }),
        };

        mockPairContract = {
            getReserves: jest.fn().mockResolvedValue([
                {
                    isZero: jest.fn().mockReturnValue(false),
                    div: jest.fn().mockReturnValue({ toNumber: () => 0.5 })
                },
                {
                    isZero: jest.fn().mockReturnValue(false),
                    div: jest.fn().mockReturnValue({ toNumber: () => 2 })
                },
                1234567890
            ]),
        };

        ethers.providers.JsonRpcProvider.mockReturnValue(mockProvider);
        ethers.Contract.mockReturnValue(mockPairContract);

        const dexAddresses = {
            uniswap: '0x1234567890123456789012345678901234567890',
            sushiswap: '0x0987654321098765432109876543210987654321',
        };

        priceMonitor = new PriceMonitor('http://localhost:8545', dexAddresses);
    });

    test('initializePairs creates pair contracts', async () => {
        await priceMonitor.initializePairs();
        expect(ethers.Contract).toHaveBeenCalledTimes(2);
    });

    test('getPrice returns correct price', async () => {
        await priceMonitor.initializePairs();
        const price = await priceMonitor.getPrice('uniswap');
        expect(price).toBeCloseTo(0.5, 5);
    });

    test('monitorPrices calls callback with prices', async () => {
        jest.useFakeTimers();
        const callback = jest.fn();
        await priceMonitor.initializePairs();
        const intervalId = await priceMonitor.monitorPrices(callback, 100);
        
        expect(callback).toHaveBeenCalledWith(expect.objectContaining({
            uniswap: expect.any(Number),
            sushiswap: expect.any(Number),
        }));
        
        clearInterval(intervalId);
        jest.useRealTimers();
    });

    test('detectArbitrageOpportunity returns opportunities', () => {
        const prices = { uniswap: 100, sushiswap: 103 };
        detectArbitrageOpportunity.mockReturnValue([['sushiswap', 103]]);
        const opportunities = priceMonitor.detectArbitrageOpportunity(prices, 0.02);
        expect(opportunities).toEqual([['sushiswap', 103]]);
        expect(detectArbitrageOpportunity).toHaveBeenCalledWith(prices, 0.02);
    });

    test('detectArbitrageOpportunity returns null when no opportunities', () => {
        const prices = { uniswap: 100, sushiswap: 101 };
        detectArbitrageOpportunity.mockReturnValue(null);
        const opportunities = priceMonitor.detectArbitrageOpportunity(prices, 0.02);
        expect(opportunities).toBeNull();
        expect(detectArbitrageOpportunity).toHaveBeenCalledWith(prices, 0.02);
    });
});
