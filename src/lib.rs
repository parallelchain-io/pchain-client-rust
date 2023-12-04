
//! Rust client library for the ParallelChain Protocol fullnode
//! [RPC API](https://github.com/parallelchain-io/parallelchain-protocol/blob/master/RPC.md).
//! 
//! ## Getting started
//! 
//! Get started by creating an instance of client. 
//! 
//! ```no_run
//! use pchain_client::{ClientV1, ClientV2};
//! 
//! // Client for RPC v1
//! let client_v1 = ClientV1::new("https://rpc_base_url.xyz");
//! 
//! // Client for RPC v2
//! let client_v2 = ClientV2::new("https://rpc_base_url.xyz");
//! ```
//! 
//! You will then be able to access each RPC through a corresponding method of the same name.
//! 
//! ```no_run
//! client_v1.submit_transaction(txn);
//! client_v1.block(block_request);
//! client_v1.state(state_request);
//! // etc.
//! ```

mod client;
pub use client::v1::ClientV1;
pub use client::v2::ClientV2;

mod networking;
pub use networking::NetworkProvider;

mod error;

