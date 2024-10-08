// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "./interfaces/IFlashLoanReceiver.sol";
import "./interfaces/IUniswapV2Router02.sol";
import "./imports/ReentrancyGuard.sol";
import "./interfaces/IPool.sol";
import "./interfaces/AggregatorV3Interface.sol";


abstract contract ArbitrageBot is Ownable, ReentrancyGuard, IFlashLoanReceiver {
    using SafeERC20 for IERC20;

    IUniswapV2Router02 public immutable uniswapRouter;
    IPool public immutable POOL;
    address public constant WETH = 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;
    mapping(address => AggregatorV3Interface) public priceFeeds;
    mapping(address => bool) public supportedTokens;

    event ArbitrageExecuted(address[] path, uint256 profit);
    event FlashLoanExecuted(address token, uint256 amount);
    event TokenAdded(address token);
    event TokenRemoved(address token);
    event PriceFeedAdded(address token, address priceFeed);
    event ErrorOccurred(string message, string reason);

    constructor(address _uniswapRouter, address _poolAddress, address initialOwner) Ownable(initialOwner) {
        uniswapRouter = IUniswapV2Router02(_uniswapRouter);
        POOL = IPool(_poolAddress);
    }


    function executeArbitrage(
        address[] memory path,
        uint256 amountIn,
        uint256 minAmountOut,
        uint256 deadline
    ) external onlyOwner nonReentrant {
        require(block.timestamp <= deadline, "Transaction expired");
        require(path.length >= 2, "Invalid path");
        require(supportedTokens[path[0]] && supportedTokens[path[path.length - 1]], "Unsupported tokens");

        IERC20(path[0]).safeTransferFrom(msg.sender, address(this), amountIn);
        IERC20(path[0]).approve(address(uniswapRouter), amountIn);

        try uniswapRouter.swapExactTokensForTokens(
            amountIn,
            minAmountOut,
            path,
            address(this),
            deadline
        ) returns (uint256[] memory amounts) {
            uint256 profit = amounts[amounts.length - 1] - amountIn;
            require(profit > 0, "No profit");

            emit ArbitrageExecuted(path, profit);

            IERC20(path[path.length - 1]).safeTransfer(msg.sender, amounts[amounts.length - 1]);
        } catch Error(string memory reason) {
            emit ErrorOccurred("Swap failed", reason);
            revert("Swap failed");
        }
    }

    function executeFlashLoanArbitrage(
        address flashLoanToken
        // uint256 flashLoanAmount,
        // address[] memory path,
        // uint256 minAmountOut,
        // uint256 deadline
    ) external onlyOwner nonReentrant {
        require(supportedTokens[flashLoanToken], "Unsupported token");
        // bytes memory params = abi.encode(path, minAmountOut, deadline);
        // Call to flash loan provider would go here
        // For example: flashLoanProvider.flashLoan(address(this), flashLoanToken, flashLoanAmount, params);
    }

    function executeOperation(
        address[] calldata assets,
        uint256[] calldata amounts,
        uint256[] calldata premiums,
        address initiator,
        bytes calldata params
    ) external virtual override returns (bool);

    function addSupportedToken(address token) external onlyOwner {
        supportedTokens[token] = true;
        emit TokenAdded(token);
    }

    function removeSupportedToken(address token) external onlyOwner {
        supportedTokens[token] = false;
        emit TokenRemoved(token);
    }

    function addPriceFeed(address token, address priceFeed) external onlyOwner {
        priceFeeds[token] = AggregatorV3Interface(priceFeed);
        emit PriceFeedAdded(token, priceFeed);
    }

    function getLatestPrice(address token) public view returns (int) {
        (, int price,,,) = priceFeeds[token].latestRoundData();
        return price;
    }

    function withdrawToken(address token, uint256 amount) external onlyOwner {
        IERC20(token).safeTransfer(msg.sender, amount);
    }

    receive() external payable {}
}
