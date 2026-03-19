# stellar-devkit

A local Stellar/Soroban developer toolkit for simulating transactions, inspecting contracts, and debugging diagnostics — without touching the network.

## Installation
```bash
cargo install stellar-devkit
```

Or build from source:
```bash
git clone https://github.com/unixfundz/stellar-devkit
cd stellar-devkit
cargo build --release
```

## Usage
```bash
stellar-devkit simulate --wasm ./contract.wasm --function transfer --args '["GABC", "GXYZ", "100"]'
stellar-devkit events --tx <base64-xdr>
stellar-devkit inspect --wasm ./contract.wasm
stellar-devkit snapshot --rpc https://soroban-testnet.stellar.org --contract GABC... --output snapshot.json
```

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md). Check open issues for tasks labelled `good first issue`.

## License

Apache 2.0
