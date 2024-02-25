## Introduction

This Rust application is a simple peer-to-peer gossip messaging system. It allows peers to gossip messages to each other over TCP connections.

## Features

- **Random Message Generation**: Messages are generated randomly to simulate real-world scenarios.
- **Asynchronous I/O**: The application uses Tokio for asynchronous I/O operations, enabling high concurrency.
- **Command-Line Interface**: The application is configurable via command-line arguments using the `clap` crate.

## Installation

To run this application, you need to have Rust and Cargo installed on your system.

1. Clone this repository:

    ```bash
    git clone https://github.com/neanvo/gossip-rust.git
    ```

2. Navigate into the project directory:

    ```bash
    cd gossip-rust
    ```

3. Build the project using Cargo:

    ```bash
    cargo build --release
    ```

## Usage

To use the application, you can specify command-line arguments to configure the behavior of the peers.

Example usage:

Execute three bash commands into 3 distinct terminals:
```bash
./target/release/gossip-rust --period 10 --port 9001 --connect "127.0.0.1:9002,127.0.0.1:9003"
```
```bash
./target/release/gossip-rust --period 10 --port 9002 --connect "127.0.0.1:9001,127.0.0.1:9003"
```
```bash
./target/release/gossip-rust --period 10 --port 9003 --connect "127.0.0.1:9001,127.0.0.1:9002"
```
