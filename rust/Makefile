# These shell flags are REQUIRED for an early exit in case any program called by make errors!
.SHELLFLAGS=-euo pipefail -c -O extglob
SHELL := /bin/bash

.PHONY: cargo check test clean

check: cargo ## Run all checks
	@cargo check
	@cargo update
	@cargo clippy --fix --allow-dirty --allow-staged
	@cargo fmt --all --check

test: ## Run unit tests.
	@cargo test

clean: ## Clean directory.
	@cargo clean

help: ## Display this help.
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)
