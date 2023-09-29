/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

///! Client defines functions to communicate with ParallelChain RPC endpoints. 
///! Users are required to provide corresponding [request](pchain_types::rpc) specified in pchain_types
///! in order to get a correct response.

pub(crate) mod v1;
pub(crate) mod v2;
