# These shell flags are REQUIRED for an early exit in case any program called by make errors!
.SHELLFLAGS=-euo pipefail -c
SHELL := /bin/bash

.PHONY: cargo check test clean

cargo:
	@cargo check
	@cargo update

check: cargo ## Run all checks
	@cargo clippy --fix
	@rustfmt --check --edition 2021 src/**/*.rs

test: ## Run unit tests.
	@cargo test

clean: ## Clean directory.
	@cargo clean

help: ## Display this help.
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)
