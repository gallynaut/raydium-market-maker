# Raydium Market Maker

This program provides an interface to submit quotes to Phoenix using a Raydium pool's fair price.

- [] Lookup Raydium account layouts
    - [Raydium AmmInfo](https://github.com/raydium-io/raydium-amm/blob/master/program/src/state.rs#L623): Used to store the Raydium account pubkeys for the AMM

- [] Lookup Solana localnet testing flows
- [] Figure out how to parameterize raydium pool configs


## Notes

- Had to import the Raydium SDK directly to the cargo workspace because of anchor/solana dep conflicts and they dont have their repo setup for cargo imports using a git url