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
	@echo "  make install            - Install binary to $(DESTDIR) (requires sudo)"
	@echo "  make deploy             - Build and deploy (requires sudo)"
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

install: release
	@if [ ! -w $(DESTDIR) ]; then \
		echo "Error: No write permission to $(DESTDIR). Use sudo."; \
		exit 1; \
	fi
	install -D -m 755 target/release/sonic-thermalctld $(DESTDIR)/sonic-thermalctld
	@echo "Installed to $(DESTDIR)/sonic-thermalctld"

install-mellanox: release-mellanox
	@if [ ! -w $(DESTDIR) ]; then \
		echo "Error: No write permission to $(DESTDIR). Use sudo."; \
		exit 1; \
	fi
	install -D -m 755 target/release/sonic-thermalctld $(DESTDIR)/sonic-thermalctld
	@echo "Installed to $(DESTDIR)/sonic-thermalctld"

deploy:
	@if [ "$(PLATFORM)" = "mellanox" ]; then \
		sudo ./scripts/deploy.sh --build --platform mellanox --backup; \
	else \
		sudo ./scripts/deploy.sh --build --platform generic --backup; \
	fi

deploy-mellanox:
	sudo ./scripts/deploy.sh --build --platform mellanox --backup

check:
	$(CARGO) check
	$(CARGO) check --features mellanox

fmt:
	$(CARGO) fmt

clippy:
	$(CARGO) clippy -- -D warnings

all: release test

.DEFAULT_GOAL := help
