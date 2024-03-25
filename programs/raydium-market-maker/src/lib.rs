//! A program demonstrating the transfer of lamports
#![deny(missing_docs)]
#![forbid(unsafe_code)]

use solana_program::pubkey;

// Why doesnt Raydium export this?
pub const RAYDIUM_V4_PROGRAM_ID: Pubkey = pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");

// @DEV - this will be removed in the final version and an instruction will be added to register a Raydium pool
pub const RAYDIUM_AMM: Pubkey = pubkey!("");

mod entrypoint;
mod error;
pub mod processor;