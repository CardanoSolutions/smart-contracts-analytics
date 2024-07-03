# Smart Contract Analytics

## Getting Started

### Pre-requisite

- Rust toolkit `>= 1.73.0` with Cargo.
- Node.js `>= 16.15.0` with Yarn / NPM.
- Ogmios `>= v6.0.0` (local or hosted on Demeter).

### Environment

Adjust the `.envrc` to select Ogmios' source.

### Collecting mainnet scripts

```
node collect-plutus-scripts.mjs > scripts.csv
```

### Identifying Aiken validators

```
cargo run --release scripts.csv > validators.json
```

### Counting scripts usage on-chain

```
node count.mjs
```
