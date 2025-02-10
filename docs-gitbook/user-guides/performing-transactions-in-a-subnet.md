---
description: >-
  The below is a tutorial on how to issue a transaction from a wallet in a
  subnet.
---

# Performing Transactions from a Wallet in a Subnet

### Connecting to a subnet

Assuming the process of launching a custom IPC Subnet with at least one validator node is complete, the custom IPC Subnet is now available for a user (defined here as a wallet on the subnet that is does not necessarily represent a validator) to perform transactions.&#x20;

* A user would begin by launching Metamask, and manually adding the custom IPC Network to their Metamask networks list using “Add a network manually.” &#x20;

\
![](https://github.com/consensus-shipyard/docs/blob/main/assets/add_network_manually.png?raw=true)

* Name the network, include the local New RPC URL and relevant ChainID (both of which are provided when you successfully launch the validator node), and name the currency symbol (for Filecoin subnets, tFIL is used). &#x20;

\
![](https://github.com/consensus-shipyard/docs/blob/main/assets/ipc_local_subnet.png?raw=true)

* Once, the network is added successfully, "Switch to IPC Local Subnet," or whatever you named your subnet.&#x20;

![](https://github.com/consensus-shipyard/docs/blob/main/assets/network_added.png?raw=true)

* Now, an account on this custom IPC Subnet can be imported to MetaMask.  Select “+ Add account or hardware wallet.”  &#x20;

\
![](https://github.com/consensus-shipyard/docs/blob/main/assets/add_account.png?raw=true)

* Select "Import account". &#x20;

![](https://github.com/consensus-shipyard/docs/blob/main/assets/import_account.png?raw=true)

* Enter the private key for the account, which is generated when establishing the subnet, and select “import.”&#x20;

\
![](https://github.com/consensus-shipyard/docs/blob/main/assets/import.png?raw=true)

Note: The private key can be retrieved on the command line by the user that deployed the subnet using the following command:&#x20;

```
ipc % cat ~/.ipc/validator_1.sk
```

The user now has a wallet address available for transactions in the custom IPC subnet.  &#x20;

### Deploying Smart Contracts in a Subnet

Consider the deployment of a simple smart contract that issues ERC20 tokens in the IPC subnet --these tokens will be acquired by a user's IPC subnet MetaMask wallet.   The steps for achieving this are as follows:&#x20;

* Open Remix and a simple contract .sol file for issuing ERC20 tokens. &#x20;

![](https://github.com/consensus-shipyard/docs/blob/main/assets/token.png?raw=true)

* For the environment, connect the IPC Subnet using the “Injected provider - Metamask” custom network.  Connect the relevant account on the IPC Subnet.&#x20;

![](https://github.com/consensus-shipyard/docs/blob/main/assets/injected_provider.png?raw=true)

* Select the relevant account and set the gas limit. &#x20;

![](https://github.com/consensus-shipyard/docs/blob/main/assets/injected_provider.png?raw=true)

* Compile and deploy the contract.  Mint new tokens by specifying the address you want the tokens to go to, and the number of tokens you wish to mint.  Click “Transact.”

![](https://github.com/consensus-shipyard/docs/blob/main/assets/deploy_run.png?raw=true)

* Copy the token address and import the token to the MetaMask wallet on the custom IPC subnet.&#x20;

![](https://github.com/consensus-shipyard/docs/blob/main/assets/deploy_metamask.png?raw=true)

* The funds of the new token should be displayed, indicating a successful transaction was performed on the IPC subnet.

### Block Explorers

Currently, browsing a custom subnet using a block explorer is not possible.  However, a block explorer that can browse custom subnets should be released soon.   \
&#x20;
