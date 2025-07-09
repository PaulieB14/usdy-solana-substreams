# USDY Solana Substreams Makefile
# This Makefile provides convenient commands for building, testing, and deploying the project

.PHONY: help build test clean install format lint run-dev run-prod package deploy docs docker

# Default target
help:
	@echo "USDY Solana Substreams - Available Commands"
	@echo "==========================================="
	@echo ""
	@echo "Development:"
	@echo "  install     - Install dependencies and setup environment"
	@echo "  build       - Build the project for WebAssembly"
	@echo "  test        - Run all tests"
	@echo "  format      - Format code using rustfmt"
	@echo "  lint        - Run clippy for code analysis"
	@echo "  clean       - Clean build artifacts"
	@echo ""
	@echo "Running:"
	@echo "  run-dev     - Run in development mode"
	@echo "  run-prod    - Run in production mode"
	@echo "  run-gui     - Run with GUI interface"
	@echo ""
	@echo "Deployment:"
	@echo "  package     - Package substreams for distribution"
	@echo "  deploy-sql  - Deploy to SQL database"
	@echo "  deploy-graph - Deploy to The Graph"
	@echo ""
	@echo "Documentation:"
	@echo "  docs        - Generate documentation"
	@echo "  docs-open   - Generate and open documentation"
	@echo ""
	@echo "Docker:"
	@echo "  docker-build - Build Docker image"
	@echo "  docker-run   - Run in Docker container"

# Variables
CARGO = cargo
SUBSTREAMS = substreams
DOCKER = docker
GRAPH = graph

# Project configuration
PROJECT_NAME = usdy_solana_tracker
DOCKER_IMAGE = usdy-solana-substreams
ENDPOINT = mainnet.sol.streamingfast.io:443
START_BLOCK = 260000000
STOP_BLOCK = +1000

# Installation and setup
install:
	@echo "🔧 Installing dependencies and setting up environment..."
	rustup target add wasm32-unknown-unknown
	@echo "✅ Installation complete!"

# Build targets
build:
	@echo "🏗️  Building USDY Solana Substreams..."
	$(CARGO) build --release --target wasm32-unknown-unknown
	@echo "✅ Build complete!"

test:
	@echo "🧪 Running tests..."
	$(CARGO) test --verbose
	@echo "✅ Tests passed!"

format:
	@echo "🎨 Formatting code..."
	$(CARGO) fmt
	@echo "✅ Code formatted!"

lint:
	@echo "🔍 Running clippy..."
	$(CARGO) clippy -- -D warnings
	@echo "✅ Linting complete!"

clean:
	@echo "🧹 Cleaning build artifacts..."
	$(CARGO) clean
	rm -rf target/
	rm -f *.spkg
	@echo "✅ Clean complete!"

# Running substreams
run-dev:
	@echo "🚀 Running in development mode..."
	$(SUBSTREAMS) gui substreams.yaml map_usdy_events \
		-e $(ENDPOINT) \
		--start-block $(START_BLOCK) \
		--stop-block $(STOP_BLOCK) \
		--debug

run-prod:
	@echo "🚀 Running in production mode..."
	$(SUBSTREAMS) run substreams.yaml map_usdy_events \
		-e $(ENDPOINT) \
		--start-block $(START_BLOCK)

package: build
	@echo "📦 Packaging substreams..."
	$(SUBSTREAMS) pack substreams.yaml
	@echo "✅ Package created!"

docs:
	@echo "📚 Generating documentation..."
	$(CARGO) doc --no-deps --document-private-items
	@echo "✅ Documentation generated!"
