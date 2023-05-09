/// `client` is the module which exposes all features of Client to the user. This module  
/// allows the user to instantiate the Client, update the networking interface the Client 
/// and submit HTTP requests to ParallelChain.
mod client;
pub use client::Client;

/// `networking` handles networking for Client
mod networking;

/// `error` handles the error handling for Client 
mod error;

/// `utils` handles interactions with wallet, keypair generation, message signing etc.
pub mod utils;
pub use utils::*;
