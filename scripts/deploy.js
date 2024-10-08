const hre = require("hardhat");

async function main() {
  const [deployer] = await hre.ethers.getSigners();

  console.log("Deploying contracts with the account:", deployer.address);

  const uniswapRouterAddress = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D"; // Uniswap V2 Router address on Sepolia
  const aaveAddressesProviderAddress = "0x0496275d34753A48320CA58103d5220d394FF77F"; // Aave V3 Pool Addresses Provider on Sepolia


  const ConcreteArbitrageBot = await hre.ethers.getContractFactory("ConcreteArbitrageBot");
  const concreteArbitrageBot = await ConcreteArbitrageBot.deploy(
    uniswapRouterAddress,
    aaveAddressesProviderAddress,
    deployer.address
  );
  
  await concreteArbitrageBot.deployed();
  
  console.log("ConcreteArbitrageBot deployed to:", concreteArbitrageBot.address);
  
  console.log("Uniswap Router:", uniswapRouterAddress);
  console.log("Aave Addresses Provider:", aaveAddressesProviderAddress);
  console.log("Owner:", deployer.address);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
