# Aiken Analytics

## Getting Started

### Pre-requisite

- Rust toolkit `>= 1.73.0` with Cargo.
- Node.js `>= 16.15.0` with Yarn / NPM.
- Ogmios `>= v6.0.0` (local or hosted on Demeter).

### Environment

Adjust the `.envrc` to select a different NETWORK or Ogmios' source.

### Data

The scripts expect some specific data files to be available under `data`:

- `data/{NETWORK}/scripts` should contain a list of base16-encoded UPLC scripts, one per line. The list may comes from various data-source such as Kupo, Oura or even Ogmios.

- `data/aiken_validators.json` is produced from running the Rust script. It isn't network specific and ideally comes from aggregating data from all networks.

### Identifying Aiken validators

```
cargo run --release > aiken_validators.json
```

### Counting scripts usage on-chain

```
yarn start
```
