# SPEC.md

## Project Overview

This project aims to build a high-performance log ingestion pipeline in Rust. The system is responsible for monitoring log files produced by a binary, tailing them in real time, parsing each JSON entry, and ingesting the data into a MongoDB cluster. Each log file represents one hour of data, and the system must handle file rotations (new file every hour and new folder every day) while processing up to millions of entries per file.

## Goals and Objectives

- **High Throughput:** Efficiently process up to 10M+ log entries per hour.
- **Real-Time Ingestion:** Stream new log entries immediately upon arrival.
- **Robustness:** Gracefully handle file rotations and unexpected file I/O issues.
- **Scalability:** Scale horizontally if needed to handle increased log volume.
- **Reliability:** Ensure that all log entries are correctly parsed and ingested into MongoDB.
- **Timestamp Integrity:** Avoid reliance on local system timestamps by parsing timestamps directly from the log entries.

## Functional Requirements

1. **File Monitoring:**
    - Watch a given root directory (e.g., `/home/hluser/hl/data/node_trades/hourly`) for changes.
    - Detect new file creations (new hour) and new directories (new day) using an event-driven approach.
    - Use the [notify](https://github.com/notify-rs/notify) crate to capture file system events.

2. **File Tailing:**
    - Open each log file and seek to its end.
    - Continuously read new lines as they are appended.
    - Implement robust tailing to handle large files and concurrent writes.

3. **Log Parsing:**
    - Each line in the log file is a JSON object with the following example structure:
      ```json
      {
        "coin": "ENA",
        "side": "B",
        "time": "2025-03-10T11:24:42.070019819",
        "px": "0.4511",
        "sz": "2133.0",
        "hash": "0x...",
        "trade_dir_override": "Na",
        "side_info": [
          { "user": "0x...", "start_pos": "1157009.0", "oid": 78656316952, "twap_id": 545248, "cloid": null },
          { "user": "0x...", "start_pos": "-9617.0", "oid": 78656314500, "twap_id": null, "cloid": null }
        ]
      }
      ```
    - Use [serde](https://serde.rs/) and [serde_json](https://github.com/serde-rs/json) to deserialize JSON entries.
    - Validate and handle parsing errors without disrupting the ingestion pipeline.

4. **MongoDB Ingestion:**
    - Insert each parsed log entry into a MongoDB cluster.
    - Use the [mongodb Rust driver](https://github.com/mongodb/mongo-rust-driver) (asynchronous variant preferred) for database operations.
    - Manage back-pressure and transient MongoDB connectivity issues.

## Non-Functional Requirements

- **Performance:**  
  The system must handle large files (up to 10M+ entries) without significant performance degradation.

- **Robustness:**  
  Proper error handling for file I/O, JSON parsing, and MongoDB insertion. The system must be resilient to file rotations and temporary failures.

- **Configurability:**  
  Support configuration through environment variables or a configuration file. Configurable parameters include:
    - Root directory path to monitor.
    - MongoDB connection string and database/collection names.
    - Polling intervals and timeouts for file tailing.

- **Logging and Monitoring:**  
  Implement logging for events, errors, and important state changes. Consider integrating with external monitoring systems for production deployments.

## Architecture Overview

### Components

1. **File Watcher Module:**
    - Monitors directories using the `notify` crate.
    - Detects file creation, modification, and new folder events.

2. **Tailer Module:**
    - Opens files and seeks to the end.
    - Continuously reads new log lines using buffered I/O.
    - Handles file rotation events gracefully.

3. **Parser Module:**
    - Uses `serde_json` to deserialize log lines.
    - Validates JSON structure and handles parse errors.

4. **MongoDB Ingestion Module:**
    - Handles connection pooling and document insertion into MongoDB.
    - Uses asynchronous operations (via `tokio`) for efficient I/O.

5. **Configuration Module:**
    - Loads and validates configuration parameters from a file or environment variables.

6. **Error Handling and Logging Module:**
    - Centralizes error reporting and logging.
    - Ensures non-critical errors do not halt processing.

### Data Flow

1. **Detection:**  
   The File Watcher module detects a new or modified log file.

2. **Processing:**  
   The Tailer module starts reading new log lines and passes them to the Parser module.

3. **Parsing:**  
   The Parser module deserializes each line into a Rust struct and verifies its validity.

4. **Ingestion:**  
   Parsed entries are sent to the MongoDB Ingestion module, which inserts them into the designated collection.

## Dependencies and Tooling

- **Crates/Libraries:**
    - `notify` (for file system events)
    - `serde` and `serde_json` (for JSON parsing)
    - `mongodb` (for MongoDB connectivity; use the async driver)
    - `tokio` (for asynchronous runtime)
    - Optionally, logging crates like `log` and `env_logger` or `tracing` for structured logging.

- **Development Tools:**
    - Cargo for building and dependency management.
    - Rustfmt and Clippy for code formatting and linting.
    - Unit and integration tests to validate functionality.

## Implementation Details

- **Error Handling:**  
  Each module should implement robust error handling. Temporary issues (e.g., MongoDB connectivity) should trigger retries with backoff strategies. Critical errors should be logged and alerted.

- **Concurrency:**  
  Use asynchronous programming (with `tokio`) to allow concurrent processing of multiple files. Consider using worker pools for database insertion to prevent bottlenecks.

- **File Rotation:**  
  The tailing logic must detect when a file is rotated (e.g., a new file is created for the next hour) and gracefully close the old file while starting to monitor the new file.

- **Timestamp Handling:**  
  Each log entry contains its own timestamp. Parsing should be based on the log content rather than local system time to avoid discrepancies.

## Performance and Scalability

- **High Throughput:**  
  Leverage Rust's native performance and asynchronous processing to handle millions of entries per hour.
- **Resource Management:**  
  Efficiently manage file descriptors and memory to prevent leaks or resource exhaustion.
- **Scalable Architecture:**  
  Design the system so that it can be horizontally scaled if log volumes increase.

## Testing and Validation

- **Unit Tests:**  
  Write unit tests for each module (e.g., parsing, file reading, and MongoDB insertion).
- **Integration Tests:**  
  Simulate file rotations and verify that the pipeline continues to process new files correctly.
- **Performance Tests:**  
  Create benchmark tests to simulate high log entry rates and measure the system’s throughput and latency.
- **Error Simulation:**  
  Test scenarios with malformed JSON, file access errors, and MongoDB downtime to validate error handling and recovery.

## Future Enhancements

- **Dynamic Scaling:**  
  Implement a dynamic worker pool for MongoDB ingestion based on load.
- **Advanced Monitoring:**  
  Integrate with monitoring tools (e.g., Prometheus) to track ingestion metrics and alert on issues.
- **Configuration Reloading:**  
  Allow dynamic reloading of configuration without restarting the service.
- **Distributed Processing:**  
  Explore options for distributed log processing to further improve scalability and fault tolerance.
