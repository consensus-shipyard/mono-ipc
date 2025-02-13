# Targets that are commands.
COMMANDS :=  deploy-stack gen compile-abi rust-binding slither lint fmt deps build \
             test install-dev install-npm-package install-eth-abi storage clean coverage \
             prepare forge

# Targets that are prerequisite commands.
PREREQ_COMMANDS := pnpm

# Targets that are not commands.
NON_COMMANDS := node_modules test/helpers/SelectorLibrary.sol

# Declare commands and prerequisite commands as phony targets.
.PHONY: $(COMMANDS) $(PREREQ_COMMANDS)

# Make all commands and non-command targets dependent on the prerequisites.
$(COMMANDS): $(PREREQ_COMMANDS)
$(NON_COMMANDS): $(PREREQ_COMMANDS)

# Override PROFILE env var to choose between `local | ci`
PROFILE ?= local

# Set to `--push` to push the multiarch image during the build.
# Leave on `--load` for local build, but it only works for a single platform.
BUILDX_STORE ?= --load
# Set according to what kind of `--platform` and `--cache` to use.
# Leave empty for local builds, then the platform matches the local one.
BUILDX_FLAGS ?=
# Set to the `<repo>/<image>:<tag>` label the image.
BUILDX_TAG   ?= contracts:latest

COMMIT_SHA := $(shell git rev-parse --short HEAD)


# To use `buildx` locally to produce multiplatform images, one needs to run `docker buildx create --use`.
# After that it looks like even the regular docker build needs the `--load` parameter, which hopefully
# it can handle for anyone with `DOCKER_BUILDKIT=1`.
docker-build:
	if [ "$(PROFILE)" = "ci" ]; then \
		docker buildx build \
			$(BUILDX_STORE) \
			$(BUILDX_FLAGS) \
			--build-arg COMMIT_SHA="$(COMMIT_SHA)" \
			-f docker/Dockerfile \
			-t $(BUILDX_TAG) $(PWD)/.; \
	else \
		DOCKER_BUILDKIT=1 \
		docker build \
			$(BUILDX_STORE) \
			--build-arg COMMIT_SHA="$(COMMIT_SHA)" \
			-f docker/Dockerfile \
			-t contracts:latest $(PWD)/.; \
	fi

pnpm:
	@command -v pnpm > /dev/null || { \
		echo "pnpm is not installed. Installing pnpm..."; \
		npm install -g pnpm; \
	}


# ==============================================================================
# Deployment

NETWORK ?= auto

# Output for generated Solidity artifacts.
# It is required by docker builds, but shouldn't be checked into git.
OUTPUT ?= out

deploy-stack:
	pnpm exec hardhat deploy-stack --network $(NETWORK)

# ==============================================================================
# Code generation

# Regenerate bindings and ABI artifacts when sources change with the `make .gen` target.
.gen: $(IPC_ACTORS_CODE)
	$(MAKE) gen
	touch $@

gen: compile-abi rust-binding

compile-abi: node_modules | forge
	mkdir -p $(OUTPUT)
	./ops/compile-abi.sh $(OUTPUT)

rust-binding:
	OUTPUT=$(OUTPUT) cargo build --locked --release --manifest-path ../crates/contract-bindings/Cargo.toml -p ipc_actors_abis

# ==============================================================================
# Running security checks within the local computer

slither:
	slither . --config-file ./slither.config.json

# ==============================================================================
# Development support

lint: fmt
	pnpm exec solhint 'contracts/**/*.sol' 'sdk/**/**/*.sol'

fmt:
	pnpm install --silent --no-save
	pnpm exec prettier --check -w 'contracts/**/**/*.sol' 'sdk/**/**/*.sol' 'test/**/**/*.sol' 'test/**/**/*.t.sol' '**/*.{js,jsx,ts,tsx,json,css}'

deps: node_modules

node_modules: package.json pnpm-lock.yaml
	pnpm install
	touch node_modules

build: node_modules | forge
	forge build
	FOUNDRY_SRC=sdk forge build

test: node_modules test/helpers/SelectorLibrary.sol | forge
	forge test -vvv --ffi

storage:
	rm -rf ./cache
	rm -rf ./cache_hardhat
	pnpm exec hardhat storage-layout --update

clean:
	rm -rf ./artifacts
	rm -rf ./cache
	rm -rf ./cache_hardhat
	rm -rf ./typechain

coverage: node_modules | forge
	forge coverage --ffi --report lcov -C ./src
	@if [ "$(shell uname)" = "Darwin" ]; then \
		genhtml -o coverage_report lcov.info --branch-coverage --ignore-errors category; \
	else \
		genhtml -o coverage_report lcov.info --branch-coverage; \
	fi
	./tools/check_coverage.sh

prepare: test/helpers/SelectorLibrary.sol fmt lint test slither

test/helpers/SelectorLibrary.sol: | forge
	pnpm exec hardhat gen-selector-library
	pnpm exec prettier -w test/helpers/SelectorLibrary.sol

# Forge is used by the ipc-solidity-actors compilation steps.
forge:
	@if [ -z "$(shell which forge)" ]; then \
		echo "Please install Foundry. See https://book.getfoundry.sh/getting-started/installation"; \
		exit 1; \
	fi

# ==============================================================================
