//! [Monocypher](https://monocypher.org) is a cryptographic library.
//!
//! It provides functions for authenticated encryption, hashing, password key derivation,
//! key exchange, and public key signatures.
//!
//! Visit the official [documentation](https://monocypher.org/manual/) for details.

extern crate hex;
extern crate libc;
extern crate monocypher_sys as ffi;

pub mod aead;
pub mod hashing;
pub mod password;
pub mod pubkey;
pub mod utils;

pub mod key_exchange;
pub mod poly1305;
