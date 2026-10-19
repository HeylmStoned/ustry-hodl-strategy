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

## Build

```bash
rustup target add wasm32-unknown-unknown
stellar contract build
```

## Test

```bash
cargo test
```

16 tests cover deposit/withdraw accounting, auth, issuer clawback and freeze, and storage TTL under mainnet parameters.

Constructor args are the USTRY SAC and an empty `init_args` vector. USTRY is the classic asset `USTRY:GCRYUGD5NVARGXT56XEZI5CIFCQETYHAPQQTHO2O3IQZTHDH4LATMYWC` as a Soroban token.
