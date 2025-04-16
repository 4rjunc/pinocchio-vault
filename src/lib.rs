//#![feature(const_mut_refs)]
#![no_std]

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

#[cfg(feature = "std")]
extern crate std;

pub mod errors;
pub mod instructions;
pub mod states;

pinocchio_pubkey::declare_id!("HhEs5ZBwrR29fQNxELBFdRN7mAvhJNP1R6xgNNL2ZkSD");
