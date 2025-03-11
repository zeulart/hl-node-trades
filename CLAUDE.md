# Hyperliquid Node Trades Listener

## Commands
- Build: `cargo build`
- Run: `cargo run`
- Build release: `cargo build --release`
- Test all: `cargo test`
- Test single: `cargo test test_name`
- Test with logs: `RUST_LOG=debug cargo test`
- Lint: `cargo clippy -- -D warnings`
- Format: `cargo fmt --all`
- Check format: `cargo fmt --all -- --check`
- Doc: `cargo doc --open`
- Benchmark: `cargo bench`

## Implementation Guidelines
- Check if tasks are already done before implementing
- Run tests before committing to ensure functionality
- Fix all warnings before committing
- Commit after each completed task
- Properly close resources (file handles, connections)
- Update documentation to reflect progress

## Code Style
- Use Rust 2024 edition with async/await (tokio runtime)
- Structure code into modules: file_watcher, tailer, parser, mongodb, config
- Follow standard Rust naming conventions (snake_case for variables/functions, CamelCase for types)
- Organize imports: standard lib first, then external crates, then internal modules
- Use strongly typed structs with serde for JSON parsing
- Implement proper error handling with custom Error types and propagation
- Use Result/Option types with ? operator for concise error handling
- Implement structured logging with appropriate levels (trace, debug, info, warn, error)
- Document all public functions and types with /// comments

## Architecture Principles
- Design for high throughput (10M+ entries/hour) and reliability
- Create unified error type system using thiserror
- Implement strict type safety for all JSON data 
- Establish backpressure/rate limiting for database operations
- Add metrics collection for performance monitoring
- Implement graceful shutdown to complete in-flight operations
- Add thorough validation for all parsed data
- Manage memory usage with appropriate buffering strategies
- Include examples in documentation
- Implement both unit and integration tests