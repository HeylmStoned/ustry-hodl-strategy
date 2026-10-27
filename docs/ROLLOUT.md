# Rollout evidence (mainnet)

Snapshot: 2026-10-23, from the Prism production database and Horizon. Every row is a One-Click account
(NEAR Chain Signatures, `chain_signatures` mode) created by a distinct EVM wallet on [prismfi.cc/earn](https://prismfi.cc/earn).
EVM addresses are omitted for privacy; the Stellar account links are the on-chain evidence.

## Totals

| Metric | Value |
| --- | --- |
| One-Click accounts created | 11 (excludes one zero-address probe, see below) |
| Accounts with a completed deposit | 9 |
| Accounts holding / having held USTRY | 8 |
| Completed deposits | 91 (Base 63, Ethereum 16, Arbitrum 12), $134.77 |
| Completed redeems | 23 (15 to Stellar USDC, 8 to EVM via CCTP), $81.17 |

## Accounts with activity

| # | Stellar account | First deposit | Deposits (USD) | Source chains | Redeems | USTRY transaction |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | [GAVK…GY2H](https://stellar.expert/explorer/public/account/GAVKXUSKJGXROYGKS7SFVPUFR33AMPWFLKPB2SZUO2HQURPFUWIHGY2H) | 2026-07-20 | 18 ($26.86) | Base, Ethereum, Arbitrum | 3 | [484bc27d…](https://stellar.expert/explorer/public/tx/484bc27dbc2f56b78f4cf991660323912ae863f20d45da05ed45a942c3819f06) |
| 2 | [GCDK…6UYZ](https://stellar.expert/explorer/public/account/GCDKLU76LJW27S6IC5VUMIZNTFRLZXGLOX2XETKHGMLEELW42J536UYZ) | 2026-07-20 | 1 ($0.50) | Base | 0 | — (USDC only) |
| 3 | [GDHG…JUL6](https://stellar.expert/explorer/public/account/GDHG3VMRXR2MUJH433PJUT5VWBI2XTZ4Y2POJKTMIOTFIVKLJZ6CJUL6) | 2026-07-21 | 23 ($18.79) | Arbitrum, Base, Ethereum | 1 | [94c21cb8…](https://stellar.expert/explorer/public/tx/94c21cb81a9bdce79c7a954f237600e45eb5034a7ecee9cb0eab4dcb9546288a) |
| 4 | [GDNW…AOAD](https://stellar.expert/explorer/public/account/GDNWVMBLSDQXMRWSPFDDO2QPO7Q4Q4S7FOFYZXJDDYJR2YF5HTB6AOAD) | 2026-07-21 | 7 ($9.98) | Arbitrum, Ethereum, Base | 4 | [b0428cb5…](https://stellar.expert/explorer/public/tx/b0428cb58e119cb295b3243a1dfdee14c5add83f2be3747a4c76a0ae97093135) |
| 5 | [GBNA…LC73](https://stellar.expert/explorer/public/account/GBNAIWDKCUESCHWVV5ZY5P4G2CHNGRZOM5PNBUSOLBTVDFX6SVV4LC73) | 2026-07-22 | 5 ($6.14) | Base | 1 | [3c24ff37…](https://stellar.expert/explorer/public/tx/3c24ff37f5e8fd19c35e43c6d003aba97df1417af05b0b331089d933da4d6a06) |
| 6 | [GCJT…L3ZX](https://stellar.expert/explorer/public/account/GCJTC2GOYEZ6NYJBHWZ4C2JKQLTFS2AVFMO5AJ6OLWSY6JRL2I2FL3ZX) | 2026-07-23 | 27 ($58.40) | Base, Arbitrum, Ethereum | 11 | [e7e0c57a…](https://stellar.expert/explorer/public/tx/e7e0c57a2b710da9dab76bb6f16351942d3e6ebd833fd56ce0a9b10f0059d39b) |
| 7 | [GBNY…5FMB](https://stellar.expert/explorer/public/account/GBNYFVDPFRTXKOZLHF2FR3NUFPFLZRCMRC2OL7XO5KSVD4YPYPWZ5FMB) | 2026-07-24 | 7 ($7.00) | Base | 0 | [add0eecd…](https://stellar.expert/explorer/public/tx/add0eecdaf111db7cd8bcaf652489a20a3f6533b9dd6b729cc8b864a8660c3f7) |
| 8 | [GCSE…R7DA](https://stellar.expert/explorer/public/account/GCSE65WDSG2NHU25OVRV56ZQSINOPC22MUJRY5KXEHKKGRFEH7XYR7DA) | 2026-08-10 | 2 ($7.00) | Base | 1 | [b79f934a…](https://stellar.expert/explorer/public/tx/b79f934a686d2542bbdc905486f7e3dde21d3cbcf60f6a7e47f26418070dbedb) |
| 9 | [GDJJ…5ZPC](https://stellar.expert/explorer/public/account/GDJJO3BILT67XCKI7EPRZWRZ2F3RQCF4H5TJEFV3R5KNLRZ6SAEC5ZPC) | 2026-10-20 | 1 ($0.10) | Base | 2 | [71336fe4…](https://stellar.expert/explorer/public/tx/71336fe4281318f39384b73a34cf830b89ecc1893dbef246db50dd497994b36d) |

## Accounts without a deposit

| Stellar account | Created | Note |
| --- | --- | --- |
| [GAV7…GGMU](https://stellar.expert/explorer/public/account/GAV7MZWOZUHOQST2MNIIGZFBBPXOPZYGSWKFW5VVF5NGJZLZ4RPSGGMU) | 2026-07-20 | Account opened, no deposit |
| [GCWK…NEQ5](https://stellar.expert/explorer/public/account/GCWKGPAZQCQWBRK6DAFJWKTSMFV4VJKABKCV2OBBZSD6GLDWIK2SNEQ5) | 2026-08-05 | Account opened, no deposit |
| [GBOV…T3B7](https://stellar.expert/explorer/public/account/GBOVI6GZ4GXN4EPOLXXOXHHX65Z7KZ53ERKY4UITHL2FF5PX6EPRT3B7) | 2026-10-22 | API probe using the zero EVM address, not a user. It led to wallet-session auth on all Stellar write routes |
