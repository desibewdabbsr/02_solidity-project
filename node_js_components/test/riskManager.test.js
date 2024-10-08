
// test/riskManager.test.js
const RiskManager = require('../src/riskManager');
const { ethers } = require('ethers');

describe('RiskManager', () => {
  let riskManager;

  beforeEach(() => {
    riskManager = new RiskManager(ethers.utils.parseEther('1000'), 0.1);
  });

  test('calculatePositionSize respects maxPositionSize', () => {
    const opportunity = { buyDex: 'uniswap', sellDex: 'sushiswap' };
    const marketLiquidity = ethers.utils.parseEther('10000');
    const positionSize = riskManager.calculatePositionSize(opportunity, marketLiquidity);
    expect(positionSize.lte(ethers.utils.parseEther('1000'))).toBe(true);
  });

  test('checkCircuitBreakers triggers when volatility exceeds threshold', () => {
    expect(riskManager.checkCircuitBreakers(0.2)).toBe(true);
    expect(riskManager.checkCircuitBreakers(0.05)).toBe(false);
  });
});
