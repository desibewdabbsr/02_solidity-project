const { ethers } = require('ethers');
const TradeExecutor = require('../src/tradeExecutor');

jest.mock('ethers');
jest.mock('../src/utils/logger');

describe('TradeExecutor', () => {
  let tradeExecutor;
  let mockProvider;
  let mockUniswapRouter;
  let mockFlashLoanProvider;

  beforeEach(() => {
      const mockBigNumber = (value) => ({
        eq: jest.fn().mockReturnValue(value === 0),
        mul: jest.fn().mockImplementation((other) => mockBigNumber(value * other)),
        div: jest.fn().mockImplementation((other) => mockBigNumber(Math.floor(value / other))),
        toString: jest.fn().mockReturnValue(value.toString()),
      });
      

    ethers.BigNumber.from = jest.fn().mockImplementation(mockBigNumber);
    ethers.constants = { Zero: mockBigNumber(0) };

    mockProvider = {
      getSigner: jest.fn(),
      getGasPrice: jest.fn().mockResolvedValue(mockBigNumber(20000000000)),
    };
    mockUniswapRouter = {
      connect: jest.fn().mockReturnThis(),
      swapExactTokensForTokens: jest.fn().mockResolvedValue({ wait: jest.fn().mockResolvedValue({ transactionHash: '0x123' }) }),
    };
    mockFlashLoanProvider = {
      connect: jest.fn().mockReturnThis(),
      flashLoan: jest.fn().mockResolvedValue({ wait: jest.fn().mockResolvedValue({ transactionHash: '0x456' }) }),
    };

    ethers.providers.JsonRpcProvider.mockReturnValue(mockProvider);
    ethers.Contract.mockImplementation((address, abi, provider) => {
      if (JSON.stringify(abi) === JSON.stringify(['function flashLoan(address,uint256,bytes)'])) {
        return mockFlashLoanProvider;
      }
      return mockUniswapRouter;
    });

    tradeExecutor = new TradeExecutor('http://localhost:8545', '0xUniswapRouterAddress', '0xFlashLoanProviderAddress');
  });

  test('executeTrade handles errors gracefully', async () => {
    const path = ['0xToken1', '0xToken2'];
    const amount = ethers.BigNumber.from('1000000000000000000');
    const minAmountOut = ethers.BigNumber.from('990000000000000000');
    const deadline = Math.floor(Date.now() / 1000) + 300;
    const account = '0xUserAddress';

    mockUniswapRouter.swapExactTokensForTokens.mockRejectedValue(new Error('Swap failed'));

    await expect(tradeExecutor.executeTrade(path, amount, minAmountOut, deadline, account))
      .rejects.toThrow('Swap failed');
  });

  test('executeFlashLoanTrade handles errors gracefully', async () => {
    const token = '0xTokenAddress';
    const amount = ethers.BigNumber.from('1000000000000000000');
    const data = '0x1234';

    mockFlashLoanProvider.flashLoan.mockRejectedValue(new Error('Flash loan failed'));

    await expect(tradeExecutor.executeFlashLoanTrade(token, amount, data))
      .rejects.toThrow('Flash loan failed');
  });

  test('calculateSlippage handles zero expected price', () => {
    const expectedPrice = ethers.BigNumber.from(0);
    const slippageTolerance = 10;

    const result = tradeExecutor.calculateSlippage(expectedPrice, slippageTolerance);

    expect(result.toString()).toEqual('0');
  });

  test('getOptimalGasPrice handles provider error', async () => {
    mockProvider.getGasPrice.mockRejectedValue(new Error('Failed to fetch gas price'));

    await expect(tradeExecutor.getOptimalGasPrice()).rejects.toThrow('Failed to fetch gas price');
  });

  test('getDeadline handles negative minutes', () => {
    const minutes = -5;
    const result = tradeExecutor.getDeadline(minutes);
  
    expect(result).toBeGreaterThanOrEqual(Math.floor(Date.now() / 1000));
  });

  test('executeTrade calls swapExactTokensForTokens with correct parameters', async () => {
    const path = ['0xToken1', '0xToken2'];
    const amount = ethers.BigNumber.from('1000000000000000000');
    const minAmountOut = ethers.BigNumber.from('990000000000000000');
    const deadline = Math.floor(Date.now() / 1000) + 300;
    const account = '0xUserAddress';

    const mockSigner = { address: account };
    mockProvider.getSigner.mockReturnValue(mockSigner);

    await tradeExecutor.executeTrade(path, amount, minAmountOut, deadline, account);

    expect(mockUniswapRouter.swapExactTokensForTokens).toHaveBeenCalledWith(
      amount,
      minAmountOut,
      path,
      account,
      deadline,
      expect.objectContaining({
        gasLimit: 300000,
        gasPrice: expect.any(Object),
      })
    );
  });

  test('executeFlashLoanTrade calls flashLoan with correct parameters', async () => {
    const token = '0xTokenAddress';
    const amount = ethers.BigNumber.from('1000000000000000000');
    const data = '0x1234';

    const mockSigner = { address: '0xUserAddress' };
    mockProvider.getSigner.mockReturnValue(mockSigner);

    await tradeExecutor.executeFlashLoanTrade(token, amount, data);

    expect(mockFlashLoanProvider.flashLoan).toHaveBeenCalledWith(
      token,
      amount,
      data,
      expect.objectContaining({
        gasLimit: 500000,
        gasPrice: expect.any(Object),
      })
    );
  });

  test('calculateSlippage returns correct value', () => {
    const expectedPrice = ethers.BigNumber.from(1000);
    const slippageTolerance = 10; // 1%

    const result = tradeExecutor.calculateSlippage(expectedPrice, slippageTolerance);

    expect(result.toString()).toEqual('10');
  });

  test('getOptimalGasPrice returns 20% higher than current gas price', async () => {
    const result = await tradeExecutor.getOptimalGasPrice();

    expect(result.toString()).toEqual('24000000000');
  });

  test('getDeadline returns correct timestamp', () => {
    const minutes = 5;
    const result = tradeExecutor.getDeadline(minutes);

    expect(result).toBeCloseTo(Math.floor(Date.now() / 1000) + 300, -1);
  });
});
