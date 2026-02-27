# rust-log-analyzer

A lightweight command-line tool in Rust that processes log files efficiently.

## Description
This tool parses standard log lines, extracts severity levels (INFO, WARN, ERROR, DEBUG, TRACE), counts occurrences, and allows filtering logs by level or searching by keyword. It uses streaming to handle files of any size with a fast footprint.

## Usage
```bash
cargo run -- -f <path_to_log_file> [-l <level>] [-s <search_term>]
```
