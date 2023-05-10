
//! Rust client library for the ParallelChain Protocol fullnode
//! [RPC API](https://github.com/parallelchain-io/parallelchain-protocol/blob/master/RPC.md).
//! 
//! ## Getting started
//! 
//! Get started by creating an instance of client. 
//! 
//! ```no_run
//! use pchain_client::Client;
//! 
//! let client = Client::new("https://rpc_base_url.xyz");
//! ```
//! 
//! You will then be able to access each RPC through a corresponding method of the same name.
//! 
//! ```no_run
//! client.submit_transaction(txn);
//! client.block(block_request);
//! client.state(state_request);
//! // etc.
//! ```

pub mod client;
pub use client::Client;

mod networking;

mod error;

