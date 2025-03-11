# Hyperliquid Node Trades Listener

A high-performance log ingestion pipeline in Rust that monitors log files containing trade data, tails them in real-time, parses JSON entries, and ingests the data into a MongoDB cluster.

## Features

- **File Watching**: Monitors a root directory for file changes and new files
- **File Tailing**: Efficiently follows log files to process new lines as they appear
- **File Rotation Handling**: Automatically detects and handles file rotation patterns
- **Resource Management**: Properly manages file handles to prevent leaks
- **Extensible Design**: Modular architecture for easy maintenance and extension

## Project Structure

The project is organized into several modules:

- **Config**: Manages application configuration from environment variables
- **FileWatcher**: Monitors a root directory for file changes and new files (in progress)
- **Parser**: Parses JSON trade entries (in progress)
- **MongoDB**: Handles database operations (in progress)

## Getting Started

### Prerequisites

- Rust 1.75 or higher
- MongoDB instance for data storage

### Installation

1. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

2. Edit the `.env` file to match your environment:
   ```
   ROOT_DIR=/path/to/your/log/directory
   MONGODB_URI=mongodb://localhost:27017
   DATABASE=trades_db
   COLLECTION=trades
   POLLING_INTERVAL_MS=500
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

### Running

```bash
# Run with default log level (info)
./target/release/hyperliquid-node-trades-listener

# Run with debug logging
RUST_LOG=debug ./target/release/hyperliquid-node-trades-listener
```

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run with logs visible
RUST_LOG=debug cargo test

# Run specific test
cargo test test_name
```

### Code Style

This project follows standard Rust code style conventions:
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting

## Acknowledgments

- Built for Hyperliquid