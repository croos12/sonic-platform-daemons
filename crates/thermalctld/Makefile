.PHONY: help build build-mellanox build-generic test clean install deploy

PLATFORM ?= generic
DESTDIR ?= /usr/bin
CARGO := cargo

help:
	@echo "SONiC thermalctld Rust - Build Targets"
	@echo ""
	@echo "  make build              - Build for generic platform (debug)"
	@echo "  make build-mellanox     - Build for Mellanox platform (debug)"
	@echo "  make release            - Build release for generic platform"
	@echo "  make release-mellanox   - Build release for Mellanox platform"
	@echo "  make test               - Run all tests"
	@echo "  make test-mellanox      - Run tests with Mellanox feature"
	@echo "  make clean              - Clean build artifacts"
	@echo ""
	@echo "Variables:"
	@echo "  PLATFORM=<name>    - Target platform (generic, mellanox)"
	@echo "  DESTDIR=<path>     - Installation directory (default: /usr/bin)"

build:
	$(CARGO) build

build-mellanox:
	$(CARGO) build --features mellanox

build-generic:
	$(CARGO) build

release:
	$(CARGO) build --release

release-mellanox:
	$(CARGO) build --release --features mellanox

release-generic:
	$(CARGO) build --release

test:
	$(CARGO) test

test-mellanox:
	$(CARGO) test --features mellanox

clean:
	$(CARGO) clean

check:
	$(CARGO) check
	$(CARGO) check --features mellanox

fmt:
	$(CARGO) fmt

clippy:
	$(CARGO) clippy -- -D warnings

all: release test

.DEFAULT_GOAL := help
