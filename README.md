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
node collect-plutus-scripts.mjs 1>plutus_scripts.csv 2>native_scripts.json
```

### Collecting reference inputs

```
node collect-reference-inputs.mjs 1>reference_inputs.json
```

### Classifying validators

```
cargo run --release plutus_scripts.csv > validators.json
```


### Counting scripts usage on-chain

```
node count.mjs validators.json native_scripts.json reference_inputs.json
```
