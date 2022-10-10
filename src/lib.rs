#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

pub mod consts;
mod hashing;
pub mod plagiarism_rate;
pub mod pre_processing;

pub use hashing::rolling_hash::{compute_hash, RollingHash};
pub use hashing::HashStats;
