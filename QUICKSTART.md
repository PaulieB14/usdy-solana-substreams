# Quick Start Guide

Get up and running with USDY Solana Substreams in under 5 minutes!

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Git](https://git-scm.com/)

## 1. Clone and Setup

```bash
# Clone the repository
git clone https://github.com/PaulieB14/usdy-solana-substreams.git
cd usdy-solana-substreams

# Install dependencies
make install
```

## 2. Build the Project

```bash
# Build for WebAssembly
make build
```

## 3. Authenticate with Substreams

```bash
# Set up authentication
substreams auth
```

## 4. Run Your First Query

```bash
# Track USDY events in development mode
make run-dev
```

Or run manually:

```bash
substreams gui substreams.yaml map_usdy_events \
  -e mainnet.sol.streamingfast.io:443 \
  --start-block 260000000 \
  --stop-block +100
```

## 5. Explore Different Modules

```bash
# Track transactions
make run-transactions

# Monitor holder balances
make run-holders

# Generate database output
make run-database
```

## What's Next?

- 📖 Read the [full documentation](README.md)
- 🚀 Deploy to [The Graph](docs/DEPLOYMENT.md#the-graph-studio)
- 🗄️ Set up [database integration](docs/DEPLOYMENT.md#database-deployment)
- 🐳 Try [Docker deployment](docs/DEPLOYMENT.md#docker-deployment)

## Common Issues

### Build fails with missing target
```bash
rustup target add wasm32-unknown-unknown
```

### Authentication errors
```bash
substreams auth --reset
```

### Connection timeouts
Try using a different endpoint or check your internet connection.

## Getting Help

- 🐛 [Report bugs](https://github.com/PaulieB14/usdy-solana-substreams/issues/new?template=bug_report.md)
- 💡 [Request features](https://github.com/PaulieB14/usdy-solana-substreams/issues/new?template=feature_request.md)
- 📚 [Read the docs](docs/)
- 💬 Join the [Substreams Discord](https://discord.gg/substreams)

Happy streaming! 🚀
