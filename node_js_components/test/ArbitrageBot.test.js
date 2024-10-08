const { BigNumber } = require('ethers');
const ArbitrageBot = require('../src/ArbitrageBot');
const PriceMonitor = require('../src/priceMonitor');
const ArbitrageFinder = require('../src/arbitrageFinderAlgorithm');
const TradeExecutor = require('../src/tradeExecutor');
const RiskManager = require('../src/riskManager');

jest.mock('../src/priceMonitor');
jest.mock('../src/arbitrageFinderAlgorithm');
jest.mock('../src/tradeExecutor');
jest.mock('../src/riskManager');

describe('ArbitrageBot', () => {
  let arbitrageBot;
  const mockProvider = {};
  const mockDexAddresses = ['0x123', '0x456'];
  const mockUniswapRouterAddress = '0x789';
  const mockFlashLoanProviderAddress = '0xabc';
  const mockMaxPositionSize = BigNumber.from(1000);
  const mockStopLossThreshold = 0.05;

  beforeEach(() => {
    arbitrageBot = new ArbitrageBot(
      mockProvider,
      mockDexAddresses,
      mockUniswapRouterAddress,
      mockFlashLoanProviderAddress,
      mockMaxPositionSize,
      mockStopLossThreshold
    );
  });

  test('constructor initializes components correctly', () => {
    expect(arbitrageBot.priceMonitor).toBeInstanceOf(PriceMonitor);
    expect(arbitrageBot.arbitrageFinder).toBeInstanceOf(ArbitrageFinder);
    expect(arbitrageBot.tradeExecutor).toBeInstanceOf(TradeExecutor);
    expect(arbitrageBot.riskManager).toBeInstanceOf(RiskManager);
  });

  test('start method initializes pairs and monitors for opportunities', async () => {
    arbitrageBot.priceMonitor.initializePairs = jest.fn().mockResolvedValue();
    arbitrageBot.arbitrageFinder.monitorForArbitrageOpportunities = jest.fn();

    await arbitrageBot.start();

    expect(arbitrageBot.priceMonitor.initializePairs).toHaveBeenCalled();
    expect(arbitrageBot.arbitrageFinder.monitorForArbitrageOpportunities).toHaveBeenCalled();
  });

  test('handleArbitrageOpportunity executes trade when position size is non-zero', async () => {
    const mockOpportunity = {
      buyDex: '0x123',
      sellDex: '0x456',
    };
    arbitrageBot.getMarketLiquidity = jest.fn().mockReturnValue(BigNumber.from(100));
    arbitrageBot.riskManager.calculatePositionSize = jest.fn().mockReturnValue(BigNumber.from(50));
    arbitrageBot.tradeExecutor.getDeadline = jest.fn().mockReturnValue(1000);
    arbitrageBot.tradeExecutor.calculateSlippage = jest.fn().mockReturnValue(BigNumber.from(45));
    arbitrageBot.tradeExecutor.executeTrade = jest.fn().mockResolvedValue({ transactionHash: '0xdef' });

    await arbitrageBot.handleArbitrageOpportunity(mockOpportunity);

    expect(arbitrageBot.tradeExecutor.executeTrade).toHaveBeenCalled();
  });

  test('handleArbitrageOpportunity does not execute trade when position size is zero', async () => {
    const mockOpportunity = {
      buyDex: '0x123',
      sellDex: '0x456',
    };
    arbitrageBot.getMarketLiquidity = jest.fn().mockReturnValue(BigNumber.from(100));
    arbitrageBot.riskManager.calculatePositionSize = jest.fn().mockReturnValue(BigNumber.from(0));
    arbitrageBot.tradeExecutor.executeTrade = jest.fn();

    await arbitrageBot.handleArbitrageOpportunity(mockOpportunity);

    expect(arbitrageBot.tradeExecutor.executeTrade).not.toHaveBeenCalled();
  });

  test('checkCircuitBreakers stops the bot when triggered', () => {
    arbitrageBot.riskManager.calculateMarketVolatility = jest.fn().mockReturnValue(0.1);
    arbitrageBot.riskManager.checkCircuitBreakers = jest.fn().mockReturnValue(true);
    arbitrageBot.stop = jest.fn();

    arbitrageBot.checkCircuitBreakers();

    expect(arbitrageBot.stop).toHaveBeenCalled();
  });

  test('checkCircuitBreakers does not stop the bot when not triggered', () => {
    arbitrageBot.riskManager.calculateMarketVolatility = jest.fn().mockReturnValue(0.01);
    arbitrageBot.riskManager.checkCircuitBreakers = jest.fn().mockReturnValue(false);
    arbitrageBot.stop = jest.fn();

    arbitrageBot.checkCircuitBreakers();

    expect(arbitrageBot.stop).not.toHaveBeenCalled();
  });
});
