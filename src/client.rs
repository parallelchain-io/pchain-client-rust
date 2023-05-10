/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Client struct with methods corresponding to each and every fullnode RPC.

use pchain_types::serialization::{Serializable, Deserializable};
use pchain_types::blockchain::Transaction;
use crate::error::{ HttpError as PChainClientError, HttpErrorResponse };
use crate::networking::Networking;

/// [Client] defines functions to communicate with ParallelChain RPC endpoints. 
/// User requires to provide corresponding request stuct in pchain_types in order
/// to get correct response.
pub struct Client {
    /// `networking` denotes instance of reqwest::Client.
    networking: Networking
}

impl Client {
    /// `new` creates a new instance of a pchain_client given a provider.
    /// # Arguments
    /// * `rpc_base_url` - ParallelChain RPC base network URL
    /// 
    pub fn new(rpc_base_url: &str) -> Self {
        Self { networking: Networking::new(String::from(rpc_base_url)) }
    }

    /// `set_provider` assign new network provider for Client.
    /// # Arguments
    /// * `rpc_base_url` - ParallelChain RPC base network URL
    /// 
    pub fn set_provider(&mut self, rpc_base_url: &str) {
        self.networking.set_provider(rpc_base_url);
    }

    /// `provider` returns the current network provider for Client, and check if 
    /// the current provider is up. 
    /// 
    /// # Return
    /// Tuple of (provider url, boolean). If server is up, returns true, otherwise returns false.
    pub async fn is_provider_up(&self) -> (String, bool) {
        self.networking.is_provider_up().await
    }
    
    /// Submit a transaction.
    pub async fn submit_transaction(
        &self, 
        signed_tx: &Transaction
    ) -> Result<pchain_types::rpc::SubmitTransactionResponse, HttpErrorResponse> { 
        let data = Transaction::serialize(&signed_tx);  

        let raw_bytes = self
            .networking
            .post_response("submit_transaction", data)
            .await
            .map_err(|e| PChainClientError::new(e.to_string()))?; 

        let response: pchain_types::rpc::SubmitTransactionResponse = pchain_types::rpc::SubmitTransactionResponse::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;  
        
        Ok(response)
    }

    /// `state` query account data in world state.
    /// # Arguments
    /// * `request` - pchain_types::rpc::StateRequest
    /// 
    pub async fn state(
        &self, 
        request: &pchain_types::rpc::StateRequest
    ) -> Result<pchain_types::rpc::StateResponse, HttpErrorResponse> { 
        let data = pchain_types::rpc::StateRequest::serialize(request);  
        
        let raw_bytes = self
            .networking
            .post_response("state", data)
            .await
            .map_err(|e| PChainClientError::new(e.to_string()))?; 

        let state_response: pchain_types::rpc::StateResponse = pchain_types::rpc::StateResponse::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(state_response)
    }

    /// `view` send request to execute a contract view call.
    /// # Arguments
    /// * `request` - pchain_types::rpc::ViewRequest
    /// 
    pub async fn view(
        &self, 
        request: &pchain_types::rpc::ViewRequest
    ) -> Result<pchain_types::rpc::ViewResponse, HttpErrorResponse> { 
        let data = pchain_types::rpc::ViewRequest::serialize(request);  
        
        let raw_bytes = self
            .networking
            .post_response("view", data)
            .await
            .map_err(|e| PChainClientError::new(e.to_string()))?; 

        let state_response: pchain_types::rpc::ViewResponse = pchain_types::rpc::ViewResponse::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(state_response)
    }

    /// `validator_sets`query previous / current / next validator sets with or without delegators included in result
    /// # Arguments
    /// * `request` - pchain_types::rpc::ValidatorSetsRequest
    /// 
    pub async fn validator_sets(
        &self, 
        request: &pchain_types::rpc::ValidatorSetsRequest
    ) -> Result<pchain_types::rpc::ValidatorSetsResponse, HttpErrorResponse> { 
        let data = pchain_types::rpc::ValidatorSetsRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("validator_sets", data)
            .await
            .map_err(|e| PChainClientError::new(e.to_string()))?; 

        let response: pchain_types::rpc::ValidatorSetsResponse = pchain_types::rpc::ValidatorSetsResponse::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }

    /// `pools` query pools with a set of operator address, with or without stakes of each ppol
    /// # Arguments
    /// * `request` - pchain_types::rpc::PoolsRequest
    /// 
    pub async fn pools(
        &self, 
        request: &pchain_types::rpc::PoolsRequest
    ) -> Result<pchain_types::rpc::PoolsResponse, HttpErrorResponse> { 
        let data = pchain_types::rpc::PoolsRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("pools", data)
            .await
            .map_err(|e| PChainClientError::new(e.to_string()))?; 

        let response: pchain_types::rpc::PoolsResponse = pchain_types::rpc::PoolsResponse::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }

    /// `stakes` query stakes with a set of (operator address, owner address)
    /// # Arguments
    /// * `request` - pchain_types::rpc::StakesRequest
    /// 
    pub async fn stakes(
        &self, 
        request: &pchain_types::rpc::StakesRequest
    ) -> Result<pchain_types::rpc::StakesResponse, HttpErrorResponse> { 
        let data = pchain_types::rpc::StakesRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("stakes", data)
            .await
            .map_err(|e| PChainClientError::new(e.to_string()))?; 

        let response: pchain_types::rpc::StakesResponse = pchain_types::rpc::StakesResponse::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }

    /// `deposits` query deposits with a set of (operator address, owner address)
    /// # Arguments
    /// * `request` - pchain_types::rpc::DepositsRequest
    /// 
    pub async fn deposits(
        &self, 
        request: &pchain_types::rpc::DepositsRequest
    ) -> Result<pchain_types::rpc::DepositsResponse, HttpErrorResponse> { 
        let data = pchain_types::rpc::DepositsRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("deposits", data)
            .await
            .map_err(|e| PChainClientError::new(e.to_string()))?; 

        let response: pchain_types::rpc::DepositsResponse = pchain_types::rpc::DepositsResponse::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }

    /// `block` gets full block data starting from specified block hash.
    /// # Arguments
    /// * `request` - pchain_types::rpc::BlocksRequest of the block
    /// 
    pub async fn block(
        &self, 
        request: &pchain_types::rpc::BlockRequest
    ) -> Result<pchain_types::rpc::BlockResponse, HttpErrorResponse> { 
        let data = pchain_types::rpc::BlockRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("block", data)
            .await
            .map_err(|e| PChainClientError::new(e.to_string()))?; 

        let response: pchain_types::rpc::BlockResponse = pchain_types::rpc::BlockResponse::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }

    /// `block_headers` gets block headers starting from specified block hash.
    /// # Arguments
    /// * `request` - pchain_types::rpc::BlockHeadersRequest of the block
    /// 
    pub async fn block_header(
        &self, 
        request: &pchain_types::rpc::BlockHeaderRequest
    ) -> Result<pchain_types::rpc::BlockHeaderResponse, HttpErrorResponse> { 
        let data = pchain_types::rpc::BlockHeaderRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("block_header", data)
            .await
            .map_err(|e| PChainClientError::new(e.to_string()))?; 

        let response: pchain_types::rpc::BlockHeaderResponse = pchain_types::rpc::BlockHeaderResponse::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }

    /// `block_height_by_hash` gets block height by specified block hash.
    /// # Arguments
    /// * `request` - pchain_types::rpc::BlockHeightByHashRequest of the block
    /// 
    pub async fn block_height_by_hash(
        &self, 
        request: &pchain_types::rpc::BlockHeightByHashRequest
    ) -> Result<pchain_types::rpc::BlockHeightByHashResponse, HttpErrorResponse> { 
        let data = pchain_types::rpc::BlockHeightByHashRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("block_height_by_hash", data)
            .await
            .map_err(|e| PChainClientError::new(e.to_string()))?; 

        let response: pchain_types::rpc::BlockHeightByHashResponse = pchain_types::rpc::BlockHeightByHashResponse::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }

    /// `block_hash_by_height` gets block hash by specified block height.
    /// # Arguments
    /// * `request` - pchain_types::rpc::BlockHashByHeightRequest of the block
    /// 
    pub async fn block_hash_by_height(
        &self, 
        request: &pchain_types::rpc::BlockHashByHeightRequest
    ) -> Result<pchain_types::rpc::BlockHashByHeightResponse, HttpErrorResponse> { 
        let data = pchain_types::rpc::BlockHashByHeightRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("block_hash_by_height", data)
            .await
            .map_err(|e| PChainClientError::new(e.to_string()))?; 

        let response: pchain_types::rpc::BlockHashByHeightResponse = pchain_types::rpc::BlockHashByHeightResponse::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }

    /// `highest_committed_block` gets returns the latest block on ParallelChain.
    ///
    pub async fn highest_committed_block(
        &self
    ) -> Result<pchain_types::rpc::HighestCommittedBlockResponse, HttpErrorResponse> { 
        let raw_bytes = self
            .networking
            .get_response("highest_committed_block")
            .await
            .map_err(|e| PChainClientError::new(e.to_string()))?; 

        Ok(pchain_types::rpc::HighestCommittedBlockResponse::deserialize(&raw_bytes)
                .map_err(|e| PChainClientError::new(e.to_string()))?)
    }

    /// `transaction` gets transaction by specified tx hash, include receipt or not.
    /// # Arguments
    /// * `request` - pchain_types::rpc::TransactionRequest of the block
    /// 
    pub async fn transaction(
        &self, 
        request: &pchain_types::rpc::TransactionRequest
    ) -> Result<pchain_types::rpc::TransactionResponse, HttpErrorResponse> { 
        let data = pchain_types::rpc::TransactionRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("transaction", data)
            .await
            .map_err(|e| PChainClientError::new(e.to_string()))?; 

        let response: pchain_types::rpc::TransactionResponse = pchain_types::rpc::TransactionResponse::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }

    /// `receipt` gets receipt with transaction, block hash and position by specified tx hash.
    /// # Arguments
    /// * `request` - pchain_types::rpc::ReceiptRequest of the block
    /// 
    pub async fn receipt(
        &self, 
        request: &pchain_types::rpc::ReceiptRequest
    ) -> Result<pchain_types::rpc::ReceiptResponse, HttpErrorResponse> { 
        let data = pchain_types::rpc::ReceiptRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("receipt", data)
            .await
            .map_err(|e| PChainClientError::new(e.to_string()))?; 

        let response: pchain_types::rpc::ReceiptResponse = pchain_types::rpc::ReceiptResponse::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }

    /// `transaction_position` gets transaction position in block by specified tx hash.
    /// # Arguments
    /// * `request` - pchain_types::rpc::TransactionPositionRequest of the block
    /// 
    pub async fn transaction_position(
        &self, 
        request: &pchain_types::rpc::TransactionPositionRequest
    ) -> Result<pchain_types::rpc::TransactionPositionResponse, HttpErrorResponse> { 
        let data = pchain_types::rpc::TransactionPositionRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("transaction_position", data)
            .await
            .map_err(|e| PChainClientError::new(e.to_string()))?; 

        let response: pchain_types::rpc::TransactionPositionResponse = pchain_types::rpc::TransactionPositionResponse::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }
}