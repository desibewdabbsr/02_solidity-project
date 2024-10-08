require("@nomiclabs/hardhat-waffle");
require("dotenv").config();

module.exports = {
  solidity: "0.8.20",
  networks: {
    hardhat: {},
    sepolia: {
      url: process.env.SEPOLIA_RPC_URL,
      accounts: [`0x${process.env.PRIVATE_KEY}`],
    },
  },
};
