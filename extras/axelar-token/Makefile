include .env
export

.PHONY: install
install:
	forge install # installs existing dependencies

.PHONY: deps
deps:
	forge install --no-commit \
		axelarnetwork/axelar-gmp-sdk-solidity@v5.6.4 \
		axelarnetwork/interchain-token-service@v1.2.1 \
		openzeppelin/openzeppelin-contracts@v5.0.1 \
		openzeppelin/openzeppelin-contracts-upgradeable@v4.9.6

.PHONY: help
help:
	@echo "\033[0;32mGeneral Commands:\033[0m"
	@echo "deps                              - Install Foundry dependencies"
	@echo "deposit                           - Deposit tokens into the subnet"
	@echo "\033[0;32mDeployment Commands:\033[0m"
	@echo "deploy-handler-implementation     - Deploy the token handler implementation contract on the Destination Network"
	@echo "deploy-sender-implementation      - Deploy the token sender contract implementation on the Origin Network"
	@echo "deploy-handler-proxy              - Deploy the token handler proxy contract on the Destination Network"
	@echo "deploy-sender-proxy               - Deploy the token sender contract proxy on the Origin Network"
	@echo "\033[0;32mUpgrade Commands:\033[0m"
	@echo "deploy-handler-v2-implementation  - Deploy the token handler V2 implementation contract"
	@echo "deploy-sender-v2-implementation   - Deploy the token sender V2 implementation contract"
	@echo "upgrade-sender-proxy              - Upgrade the token sender contract proxy to the V2 implementation on the Origin Network"
	@echo "upgrade-handler-proxy             - Upgrade the token handler proxy contract to the V2 implementation on the Destination Network"


.PHONY: deploy-handler-implementation
deploy-handler-implementation:
	forge script ./script/Deploy.s.sol \
		--sig "deployTokenHandlerImplementation()" \
		--rpc-url "${$(DEST_NETWORK)__RPC_URL}" \
		--broadcast \
		--skip-simulation


.PHONY: deploy-handler-proxy
deploy-handler-proxy:
	forge script ./script/Deploy.s.sol \
		--sig "deployTokenHandlerProxy()" \
		--rpc-url "${$(DEST_NETWORK)__RPC_URL}" \
		--broadcast \
		--skip-simulation

.PHONY: upgrade-handler-proxy
upgrade-handler-proxy:
	forge script ./script/Upgrade.s.sol \
		--sig "upgradeTokenHandlerProxy()" \
		--rpc-url "${$(DEST_NETWORK)__RPC_URL}" \
		--broadcast \
		--skip-simulation


.PHONY: deploy-sender-implementation
deploy-sender-implementation:
	forge script ./script/Deploy.s.sol \
		--sig "deployTokenSenderImplementation()" \
		--rpc-url "${$(ORIGIN_NETWORK)__RPC_URL}" \
		--broadcast \
		--skip-simulation

.PHONY: deploy-sender-proxy
deploy-sender-proxy:
	forge script ./script/Deploy.s.sol \
		--sig "deployTokenSenderProxy()" \
		--rpc-url "${$(ORIGIN_NETWORK)__RPC_URL}" \
		--broadcast \
		--skip-simulation

.PHONY: deploy-handler-v2-implementation
deploy-handler-v2-implementation:
	forge script ./script/Upgrade.s.sol \
		--sig "deployTokenHandlerV2Implementation()" \
		--rpc-url "${$(DEST_NETWORK)__RPC_URL}" \
		--broadcast \
		--skip-simulation

.PHONY: deploy-sender-v2-implementation
deploy-sender-v2-implementation:
	forge script ./script/Upgrade.s.sol \
		--sig "deployTokenSenderV2Implementation()" \
		--rpc-url "${$(ORIGIN_NETWORK)__RPC_URL}" \
		--broadcast \
		--skip-simulation


.PHONY: upgrade-sender-proxy
upgrade-sender-proxy:
	forge script ./script/Upgrade.s.sol \
		--sig "upgradeTokenSenderProxy()" \
		--rpc-url "${$(ORIGIN_NETWORK)__RPC_URL}" \
		--broadcast \
		--skip-simulation



.PHONY: deposit
deposit:
	@echo "Please enter these details:"; \
	read -p "Beneficiary: " BENEFICIARY; \
  	read -p "Amount: " AMOUNT; \
	read -p "Token ID (as registered in Axelar): " TOKEN_ID; \
	read -p "Destination subnet root [314159]: " SUBNET_ROOT; \
	read -p "Destination subnet address: " SUBNET_ADDR; \
	read -p "Gas payment (in origin's native coin) [350000000000000000]: " GAS_PAYMENT; \
	forge script ./script/Deposit.s.sol \
		--rpc-url "${$(ORIGIN_NETWORK)__RPC_URL}" \
		--broadcast \
		--sig "run(bytes32,uint256,uint256,address,uint64,address)" \
		-vvv \
		$$TOKEN_ID $$AMOUNT $${GAS_PAYMENT:-350000000000000000} $$BENEFICIARY $${SUBNET_ROOT:-314159} $$SUBNET_ADDR