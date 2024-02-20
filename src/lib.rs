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
//! // Instantiate the Client.
//! let client = Client::new("https://rpc_base_url.xyz");
//!
//! // You will then be able to access each RPC through a corresponding method of the same name.
//! // But be careful of distinguishing the methods of RPC V1 and RPC V2.
//!
//! ```no_run
//! // RPC V1
//! client.submit_transaction_v1(txn);
//! client.block_v1(block_request);
//! client.state_v1(state_request);
//!
//! // RPC V2
//! client.submit_transaction_v2(txn);
//! client.block_v2(block_request);
//! client.state_v2(state_request);
//! ```

mod client;
pub use client::Client;

mod networking;

mod error;
