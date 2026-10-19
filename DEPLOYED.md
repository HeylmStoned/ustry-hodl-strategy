# Mainnet deployment (2026-09-15)

| Item | Address |
| --- | --- |
| Operator | `GDKDVEJZGOWWLSNBM7JUUUA4GGWFHNFPYFHM6TSKPFC44773OESC7OVT` |
| USTRY SAC | `CBLV4ATSIWU67CFSQU2NVRKINQIKUZ2ODSZBUJTJ43VJVRSBTZYOPNUR` |
| Strategy WASM hash | `86c4afab2b71ff7641a2ebfe22a9817ad2aecf6feb79c47c1ce2d4919a7e3125` |
| Strategy | `CAWOR3R7CCTVTJE3DFLONZETA6GO7DKAOVVWWGHLXU6GCQU56WNH3JPJ` |
| Vault | `CCGPKYGAZWCQVMYRAROHKOACRVNIDWLAZPF3LFYJJMGZJUZ3MJP7FZQL` |
| Factory | `CDKFHFJIET3A73A2YN4KV7NSV32S6YGQMUFH3DNJXLBWL4SKEGVRNFKI` |

- WASM upload: https://stellar.expert/explorer/public/tx/bfb774865d5b34e8d2526fdec322dd3f1258a3ab18890bdc7f716b8566134a04
- Strategy deploy: https://stellar.expert/explorer/public/tx/3bd1b128014c254ff8358273279a07546be056df90e9c932fdea5123ac566363
- Vault create: https://stellar.expert/explorer/public/tx/eee28563667fb501fbe0d2641ead5154b65587d1d38da4b2dba9720e8f65bf97
- Strategy lab: https://lab.stellar.org/r/mainnet/contract/CAWOR3R7CCTVTJE3DFLONZETA6GO7DKAOVVWWGHLXU6GCQU56WNH3JPJ
- First invest (4.6907961 USTRY): https://stellar.expert/explorer/public/tx/9f1da6eccf16a051e0ab341cb1ff7051a7d5eea437e6e0281581ff3e00534821
- Partial unwind (1 USTRY): https://stellar.expert/explorer/public/tx/7f7117f824834c1575a580bc84ff54d7841e80b1ae08cd70ce67b2171830a081

## Vault roles (2026-10-19)

| Role | Holder |
| --- | --- |
| Manager | `CAWOR3R7CCTVTJE3DFLONZETA6GO7DKAOVVWWGHLXU6GCQU56WNH3JPJ` (the strategy) |
| RebalanceManager | Operator |
| EmergencyManager | Operator |
| VaultFeeReceiver | Operator |

The vault was created with `upgradable = true`. Its Manager role was handed to the strategy contract, which is immutable, has no `__check_auth` and never calls the vault, so it can never authorise `upgrade` or any other Manager-only call. The vault code is therefore frozen, and the Manager-only role setters are permanently disabled.

- Manager handover: https://stellar.expert/explorer/public/tx/aea54053f78eca44f128da7ca541255f6383dffceeef1365f2442eb94a3fd936
- Check: `stellar contract invoke --network mainnet --id CCGPKYGAZWCQVMYRAROHKOACRVNIDWLAZPF3LFYJJMGZJUZ3MJP7FZQL -- get_manager`
