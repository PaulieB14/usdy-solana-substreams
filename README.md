# USDY Solana Substreams

Real-time USDY token analytics pipeline for Solana blockchain data.

## 🎯 Overview

This project streams all USDY (Ondo US Dollar Yield) transactions from Solana, starting from the first mint (block 290789141), and stores them in efficient Parquet files for analytics and visualization.

**Token Details:**
- Mint: `A1KLoBrKBde8Ty9qtNQUtq3C2ortoC3u7twggz7sEto6`
- Symbol: USDY
- Name: Ondo US Dollar Yield

## 🚀 Quick Start

### 1. Setup Environment

```bash
# Install dependencies
make install

# Build the project
make build
```

### 2. Configure Authentication

Create a `local.env` file with your StreamingFast JWT token:
```bash
SUBSTREAMS_API_TOKEN=your_jwt_token_here
```

### 3. Start Parquet Streaming

```bash
# Start fresh streaming
make stream-parquet

# Resume from last position
make stream-parquet-resume

# Check progress
make check-progress
```

## 📊 Data Output

The system generates:
- **Parquet chunks**: `data/usdy_parquet/usdy_chunk_XXXX.parquet`
- **Progress tracking**: `data/usdy_parquet/progress.json`

Each Parquet file contains:
- `signature`: Transaction signature
- `block_number`: Block number
- `block_timestamp`: Block timestamp
- `block_hash`: Block hash
- `event_type`: Type of USDY event
- `instruction_index`: Instruction index
- `raw_event`: Complete event data (JSON)

## 🛠️ Available Commands

### Development
- `make install` - Install dependencies and setup environment
- `make build` - Build the project for WebAssembly
- `make test` - Run all tests
- `make format` - Format code using rustfmt
- `make lint` - Run clippy for code analysis
- `make clean` - Clean build artifacts

### Running
- `make run-dev` - Run in development mode with GUI
- `make stream-parquet` - Start clean Parquet streaming
- `make stream-parquet-resume` - Resume Parquet streaming
- `make check-progress` - Check streaming progress

### Deployment
- `make package` - Package substreams for distribution

## 📁 Project Structure

```
├── src/
│   ├── lib.rs              # Main Rust substreams logic
│   └── pb/                 # Protocol buffer definitions
├── proto/
│   └── usdy_events.proto   # Event definitions
├── data/
│   └── usdy_parquet/       # Output Parquet files
├── stream_usdy_parquet.py  # Python streaming script
├── check_progress.py       # Progress monitoring
├── substreams.yaml         # Substreams configuration
├── local.env              # Environment variables (create this)
└── Makefile               # Build and run commands
```

## 🔧 Technical Details

- **Substreams**: Real-time blockchain data processing
- **Parquet**: Columnar storage format for analytics
- **Chunking**: 10,000 blocks per Parquet file
- **Resume**: Automatic progress tracking and resume capability
- **Authentication**: JWT token authentication with StreamingFast

## 📈 Analytics Ready

The Parquet files are optimized for:
- **Pandas/Python**: Direct loading with `pd.read_parquet()`
- **Apache Spark**: Distributed processing
- **DuckDB**: Fast analytical queries
- **Tableau/PowerBI**: Business intelligence tools

## 🚨 Important Notes

- Ensure your `local.env` file contains a valid JWT token
- The system automatically resumes from the last processed block
- Each chunk processes 10,000 blocks (adjustable in the script)
- Progress is saved after each successful chunk

## 📞 Support

For issues or questions, check the logs and ensure:
1. JWT token is valid and in `local.env`
2. Network connectivity to StreamingFast endpoint
3. Sufficient disk space for Parquet files

---

**Happy streaming! 🚀**
