# Smart Contract Analytics

## Getting Started

### Pre-requisite

- Rust toolkit `>= 1.73.0` with Cargo.
- Node.js `>= 16.15.0` with Yarn / NPM.
- Ogmios `>= v6.0.0` (local or hosted on Demeter).

### Environment

Adjust the `.envrc` to select Ogmios' source.

> [!CAUTION]
> Ogmios needs to be running with the `--include-cbor` flag set.

### Collecting mainnet scripts

```
node collect-plutus-scripts.mjs 1>data/plutus_scripts.csv 2>data/native_scripts/YYYY_MM__YYYY_MM.json
```

### Collecting reference inputs

```
node collect-reference-inputs.mjs 1>data/reference_scripts/YYYY_MM__YYYY_MM.json
```

### Classifying validators

```
cargo run --release -- data/plutus_scripts.csv 1>data/validators/YYYY_MM__YYYY_MM.json
```

### Counting scripts usage on-chain

```
node count.mjs
```
