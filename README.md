# Prism USTRY Hodl Strategy

Oracle-free [DeFindex](https://docs.defindex.io) strategy that holds Etherfuse **USTRY**. Yield is the issuer redemption NAV, so `harvest` does not swap or compound. MIT.

Implements the DeFindex strategy ABI: `asset`, `deposit`, `withdraw`, `balance`, `harvest`.

## Mainnet

Deployed 2026-09-15. Full record: [DEPLOYED.md](DEPLOYED.md).

| | |
| --- | --- |
| Strategy | [`CAWOR3R7CCTVTJE3DFLONZETA6GO7DKAOVVWWGHLXU6GCQU56WNH3JPJ`](https://stellar.expert/explorer/public/contract/CAWOR3R7CCTVTJE3DFLONZETA6GO7DKAOVVWWGHLXU6GCQU56WNH3JPJ) |
| Vault | [`CCGPKYGAZWCQVMYRAROHKOACRVNIDWLAZPF3LFYJJMGZJUZ3MJP7FZQL`](https://stellar.expert/explorer/public/contract/CCGPKYGAZWCQVMYRAROHKOACRVNIDWLAZPF3LFYJJMGZJUZ3MJP7FZQL) |
| USTRY SAC | `CBLV4ATSIWU67CFSQU2NVRKINQIKUZ2ODSZBUJTJ43VJVRSBTZYOPNUR` |
| WASM hash | `86c4afab2b71ff7641a2ebfe22a9817ad2aecf6feb79c47c1ce2d4919a7e3125` |

Deposits go through [prismfi.cc/earn](https://prismfi.cc/earn).

## Build (reproduces the mainnet WASM)

Pinned: Rust `1.98.1` (`rust-toolchain.toml`), `soroban-sdk 22.0.11` (`Cargo.lock`), [stellar CLI](https://github.com/stellar/stellar-cli) `28.0.0`. The same versions are recorded in the deployed contract's metadata.

```bash
cargo install --locked stellar-cli --version 28.0.0
stellar contract build
sha256sum target/wasm32v1-none/release/ustry_hodl_strategy.wasm
# 86c4afab2b71ff7641a2ebfe22a9817ad2aecf6feb79c47c1ce2d4919a7e3125
```

`rustup` installs the pinned toolchain and the `wasm32v1-none` target on first build. Use `stellar contract build`, not plain `cargo build`: the CLI writes its build metadata into the WASM, which is part of the hash.

Compare with the live contract:

```bash
stellar contract fetch --network mainnet --id CAWOR3R7CCTVTJE3DFLONZETA6GO7DKAOVVWWGHLXU6GCQU56WNH3JPJ -o deployed.wasm
sha256sum deployed.wasm
stellar contract info meta --wasm deployed.wasm
```

## Test

```bash
cargo test
```

16 tests cover deposit/withdraw accounting, auth, issuer clawback and freeze, and storage TTL under mainnet parameters.

Constructor args are the USTRY SAC and an empty `init_args` vector. USTRY is the classic asset `USTRY:GCRYUGD5NVARGXT56XEZI5CIFCQETYHAPQQTHO2O3IQZTHDH4LATMYWC` as a Soroban token.
