const { ethers } = require('ethers');
const logger = require('./utils/logger');

class TradeExecutor {
  constructor(providerUrl, uniswapRouterAddress, flashLoanProviderAddress) {
    this.provider = new ethers.providers.JsonRpcProvider(providerUrl);
    this.uniswapRouter = new ethers.Contract(uniswapRouterAddress, ['function swapExactTokensForTokens(uint256,uint256,address[],address,uint256)'], this.provider);
    this.flashLoanProvider = new ethers.Contract(flashLoanProviderAddress, ['function flashLoan(address,uint256,bytes)'], this.provider);
    logger.info('TradeExecutor initialized');
  }

  async executeTrade(path, amount, minAmountOut, deadline, account) {
    try {
      logger.info(`Executing trade: ${JSON.stringify({ path, amount: amount.toString(), minAmountOut: minAmountOut.toString(), deadline, account })}`);
      const signer = this.provider.getSigner(account);
      const gasPrice = await this.getOptimalGasPrice();
      const tx = await this.uniswapRouter.connect(signer).swapExactTokensForTokens(
        amount,
        minAmountOut,
        path,
        account,
        deadline,
        { gasLimit: 300000, gasPrice }
      );
      const receipt = await tx.wait();
      logger.info(`Trade executed successfully: ${receipt.transactionHash}`);
      return receipt;
    } catch (error) {
      logger.error(`Error executing trade: ${error.message}`, { error });
      throw error;
    }
  }

  async executeFlashLoanTrade(token, amount, data) {
    try {
      logger.info(`Executing flash loan trade: ${JSON.stringify({ token, amount: amount.toString() })}`);
      const signer = this.provider.getSigner();
      const gasPrice = await this.getOptimalGasPrice();
      const tx = await this.flashLoanProvider.connect(signer).flashLoan(
        token,
        amount,
        data,
        { gasLimit: 500000, gasPrice }
      );
      const receipt = await tx.wait();
      logger.info(`Flash loan trade executed successfully: ${receipt.transactionHash}`);
      return receipt;
    } catch (error) {
      logger.error(`Error executing flash loan trade: ${error.message}`, { error });
      throw error;
    }
  }
  
  calculateSlippage(expectedPrice, slippageTolerance) {
    try {
      if (expectedPrice.eq(ethers.constants.Zero)) {
        logger.warn('Expected price is zero, returning zero slippage');
        return ethers.constants.Zero;
      }
      const slippage = expectedPrice.mul(slippageTolerance).div(1000);
      logger.debug(`Calculated slippage: ${slippage.toString()}`);
      return slippage;
    } catch (error) {
      logger.error(`Error calculating slippage: ${error.message}`, { error });
      throw error;
    }
  }
  

  async getOptimalGasPrice() {
    try {
      const gasPrice = await this.provider.getGasPrice();
      const optimalGasPrice = gasPrice.mul(120).div(100); // 20% higher than current gas price
      logger.debug(`Optimal gas price: ${optimalGasPrice.toString()}`);
      return optimalGasPrice;
    } catch (error) {
      logger.error(`Error getting optimal gas price: ${error.message}`, { error });
      throw error;
    }
  }
  
  
  

  

  getDeadline(minutes) {
    const deadline = Math.floor(Date.now() / 1000) + Math.max(60, minutes * 60); // Ensure at least 1 minute in the future
    logger.debug(`Calculated deadline: ${deadline}`);
    return deadline;
  }

}

module.exports = TradeExecutor;
