use ethers::prelude::*;
use std::sync::Arc;
use log::{info, error};

pub struct TradeExecutor {
    provider: Arc<Provider<Http>>,
    uniswap_router: Contract<Arc<Provider<Http>>>,
    flash_loan_provider: Contract<Arc<Provider<Http>>>,
}

impl TradeExecutor {
    pub fn new(provider_url: &str, uniswap_router_address: Address, flash_loan_provider_address: Address) -> Self {
        let provider = Arc::new(Provider::<Http>::try_from(provider_url).expect("Failed to create provider"));
        
        let uniswap_router = Contract::new(
            uniswap_router_address,
            abigen!(UniswapRouter, "function swapExactTokensForTokens(uint256,uint256,address[],address,uint256)"),
            provider.clone(),
        );

        let flash_loan_provider = Contract::new(
            flash_loan_provider_address,
            abigen!(FlashLoanProvider, "function flashLoan(address,uint256,bytes)"),
            provider.clone(),
        );

        TradeExecutor {
            provider,
            uniswap_router,
            flash_loan_provider,
        }
    }

    pub async fn execute_trade(&self, path: Vec<Address>, amount: U256, min_amount_out: U256, deadline: U256, account: Address) -> Result<TransactionReceipt, Box<dyn std::error::Error>> {
        info!("Executing trade: {:?}", path);
        
        let gas_price = self.get_optimal_gas_price().await?;
        
        let tx = self.uniswap_router
            .method("swapExactTokensForTokens", (amount, min_amount_out, path, account, deadline))?
            .gas_price(gas_price)
            .send()
            .await?;

        let receipt = tx.await?;
        info!("Trade executed successfully: {:?}", receipt.transaction_hash);
        
        Ok(receipt)
    }

    pub async fn execute_flash_loan_trade(&self, token: Address, amount: U256, data: Bytes) -> Result<TransactionReceipt, Box<dyn std::error::Error>> {
        info!("Executing flash loan trade for token: {:?}, amount: {:?}", token, amount);
        
        let gas_price = self.get_optimal_gas_price().await?;
        
        let tx = self.flash_loan_provider
            .method("flashLoan", (token, amount, data))?
            .gas_price(gas_price)
            .send()
            .await?;

        let receipt = tx.await?;
        info!("Flash loan trade executed successfully: {:?}", receipt.transaction_hash);
        
        Ok(receipt)
    }

    async fn get_optimal_gas_price(&self) -> Result<U256, Box<dyn std::error::Error>> {
        let gas_price = self.provider.get_gas_price().await?;
        Ok(gas_price)
    }
}