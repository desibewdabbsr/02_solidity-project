// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./ArbitrageBot.sol";
import "./interfaces/IPool.sol";



contract ConcreteArbitrageBot is ArbitrageBot {
    constructor(address _uniswapRouter, address _poolAddress, address initialOwner) 
        ArbitrageBot(_uniswapRouter, _poolAddress, initialOwner) {}

    function executeOperation(
        address[] calldata assets,
        uint256[] calldata amounts,
        uint256[] calldata premiums,
        address initiator,
        bytes calldata params
    ) external override returns (bool) {
        require(msg.sender == address(POOL), "Caller must be lending pool");
        require(initiator == address(this), "Initiator must be this contract");

        (address[] memory path, uint256 minAmountOut, uint256 deadline) = abi.decode(params, (address[], uint256, uint256));
        require(block.timestamp <= deadline, "Transaction expired");

        try IERC20(assets[0]).approve(address(uniswapRouter), amounts[0]) {
            try uniswapRouter.swapExactTokensForTokens(
                amounts[0],
                minAmountOut,
                path,
                address(this),
                deadline
            ) returns (uint256[] memory swapAmounts) {
                uint256 amountOwing = amounts[0] + premiums[0];
                require(swapAmounts[swapAmounts.length - 1] > amountOwing, "No profit");
                IERC20(assets[0]).approve(msg.sender, amountOwing);
                emit ArbitrageExecuted(path, swapAmounts[swapAmounts.length - 1] - amountOwing);
                return true;
            } catch Error(string memory reason) {
                emit ErrorOccurred("Swap failed", reason);
                revert("Swap failed");
            }
        } catch Error(string memory reason) {
            emit ErrorOccurred("Token approval failed", reason);
            revert("Token approval failed");
        }
    }
    function ADDRESSES_PROVIDER() external view override returns (IPoolAddressesProvider) {
    return IPoolAddressesProvider(address(POOL));
}

}
