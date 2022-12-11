# yang-amm-near-demo
AMM demo build on top of NEAR chain.

## Prerequisites
To install Rust and Wasm run:
```bash
# Installing Rust in Linux and MacOS
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env

# Add the wasm toolchain
rustup target add wasm32-unknown-unknown
```

## Building
To build run:
```bash
./build.sh
```

## Testing
To test run:
```bash
cargo test --workspace -- --nocapture
```