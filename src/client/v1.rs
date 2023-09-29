/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

use async_trait::async_trait;
use pchain_types::blockchain::TransactionV1;
use pchain_types::serialization::{Serializable, Deserializable};
use crate::error::{ self as PChainClientError, HttpErrorResponse };
use crate::networking::{Networking, NetworkProvider};

/// Client with methods corresponding to each and every fullnode RPC V1.
pub struct ClientV1 {
    /// `networking` denotes instance of reqwest::Client.
    networking: Networking,
}

#[async_trait]
impl NetworkProvider for ClientV1 {
    fn set_provider(&mut self, rpc_base_url: &str) {
        self.networking.set_provider(rpc_base_url);
    }

    fn get_provider(&self) -> String {
        self.networking.get_provider()
    }

    async fn is_provider_up(&self) -> bool {
        self.networking.is_provider_up().await
    }
}

impl ClientV1 {
    /// `new` creates a new instance of a pchain_client given a network provider.
    /// # Arguments
    /// * `rpc_base_url` - base URL of Parallelchain RPC endpoints
    /// 
    pub fn new(rpc_base_url: &str) -> Self {
        Self { networking: Networking::new(String::from(rpc_base_url))}
    }
    
    /// `submit_transaction` sends request to submit a transaction.
    pub async fn submit_transaction(
        &self, 
        tx: &TransactionV1
    ) -> Result<pchain_types::rpc::SubmitTransactionResponseV1, HttpErrorResponse> { 
        let request = pchain_types::rpc::SubmitTransactionRequestV1{ transaction: tx.clone() };
        let data = pchain_types::rpc::SubmitTransactionRequestV1::serialize(&request); 

        let raw_bytes = self
            .networking
            .post_response("submit_transaction", data)
            .await
            .map_err(PChainClientError::new)?; 

        let response: pchain_types::rpc::SubmitTransactionResponseV1 = pchain_types::rpc::SubmitTransactionResponseV1::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;  
        
        Ok(response)
    }

    /// `state` sends request to query account data from world state.
    pub async fn state(
        &self, 
        request: &pchain_types::rpc::StateRequest
    ) -> Result<pchain_types::rpc::StateResponse, HttpErrorResponse> { 
        let data = pchain_types::rpc::StateRequest::serialize(request);  
        
        let raw_bytes = self
            .networking
            .post_response("state", data)
            .await
            .map_err(PChainClientError::new)?; 

        let state_response: pchain_types::rpc::StateResponse = pchain_types::rpc::StateResponse::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(state_response)
    }

    /// `view` sends request to execute a contract view call.
    pub async fn view(
        &self, 
        request: &pchain_types::rpc::ViewRequest
    ) -> Result<pchain_types::rpc::ViewResponseV1, HttpErrorResponse> { 
        let data = pchain_types::rpc::ViewRequest::serialize(request);  
        
        let raw_bytes = self
            .networking
            .post_response("view", data)
            .await
            .map_err(PChainClientError::new)?; 

        let state_response: pchain_types::rpc::ViewResponseV1 = pchain_types::rpc::ViewResponseV1::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(state_response)
    }

    /// `validator_sets` sends request to query previous / current / next validator and delegator sets
    pub async fn validator_sets(
        &self, 
        request: &pchain_types::rpc::ValidatorSetsRequest
    ) -> Result<pchain_types::rpc::ValidatorSetsResponse, HttpErrorResponse> { 
        let data = pchain_types::rpc::ValidatorSetsRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("validator_sets", data)
            .await
            .map_err(PChainClientError::new)?; 

        let response: pchain_types::rpc::ValidatorSetsResponse = pchain_types::rpc::ValidatorSetsResponse::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }

    /// `pools` sends request to query pools with a set of operator address, with or without stakes of each pool
    pub async fn pools(
        &self, 
        request: &pchain_types::rpc::PoolsRequest
    ) -> Result<pchain_types::rpc::PoolsResponse, HttpErrorResponse> { 
        let data = pchain_types::rpc::PoolsRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("pools", data)
            .await
            .map_err(PChainClientError::new)?; 

        let response: pchain_types::rpc::PoolsResponse = pchain_types::rpc::PoolsResponse::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }

    /// `stakes` sends request to query stakes with a set of (operator address, owner address)
    pub async fn stakes(
        &self, 
        request: &pchain_types::rpc::StakesRequest
    ) -> Result<pchain_types::rpc::StakesResponse, HttpErrorResponse> { 
        let data = pchain_types::rpc::StakesRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("stakes", data)
            .await
            .map_err(PChainClientError::new)?; 

        let response: pchain_types::rpc::StakesResponse = pchain_types::rpc::StakesResponse::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }

    /// `deposits` sends request to query deposits with a set of (operator address, owner address)
    pub async fn deposits(
        &self, 
        request: &pchain_types::rpc::DepositsRequest
    ) -> Result<pchain_types::rpc::DepositsResponse, HttpErrorResponse> { 
        let data = pchain_types::rpc::DepositsRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("deposits", data)
            .await
            .map_err(PChainClientError::new)?; 

        let response: pchain_types::rpc::DepositsResponse = pchain_types::rpc::DepositsResponse::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }

    /// `block` sends request to get full block data starting from specified block hash.
    pub async fn block(
        &self, 
        request: &pchain_types::rpc::BlockRequest
    ) -> Result<pchain_types::rpc::BlockResponseV1, HttpErrorResponse> { 
        let data = pchain_types::rpc::BlockRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("block", data)
            .await
            .map_err(PChainClientError::new)?; 

        let response: pchain_types::rpc::BlockResponseV1 = pchain_types::rpc::BlockResponseV1::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }

    /// `block_header` sends request to get block header starting from specified block hash.
    pub async fn block_header(
        &self, 
        request: &pchain_types::rpc::BlockHeaderRequest
    ) -> Result<pchain_types::rpc::BlockHeaderResponseV1, HttpErrorResponse> { 
        let data = pchain_types::rpc::BlockHeaderRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("block_header", data)
            .await
            .map_err(PChainClientError::new)?; 

        let response: pchain_types::rpc::BlockHeaderResponseV1 = pchain_types::rpc::BlockHeaderResponseV1::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }

    /// `block_height_by_hash` sends request to get block height by specified block hash.
    pub async fn block_height_by_hash(
        &self, 
        request: &pchain_types::rpc::BlockHeightByHashRequest
    ) -> Result<pchain_types::rpc::BlockHeightByHashResponse, HttpErrorResponse> { 
        let data = pchain_types::rpc::BlockHeightByHashRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("block_height_by_hash", data)
            .await
            .map_err(PChainClientError::new)?; 

        let response: pchain_types::rpc::BlockHeightByHashResponse = pchain_types::rpc::BlockHeightByHashResponse::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }

    /// `block_hash_by_height` sends request to get block hash by specified block height.
    pub async fn block_hash_by_height(
        &self, 
        request: &pchain_types::rpc::BlockHashByHeightRequest
    ) -> Result<pchain_types::rpc::BlockHashByHeightResponse, HttpErrorResponse> { 
        let data = pchain_types::rpc::BlockHashByHeightRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("block_hash_by_height", data)
            .await
            .map_err(PChainClientError::new)?; 

        let response: pchain_types::rpc::BlockHashByHeightResponse = pchain_types::rpc::BlockHashByHeightResponse::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }

    /// `highest_committed_block` sends request to get the latest block on ParallelChain.
    pub async fn highest_committed_block(
        &self
    ) -> Result<pchain_types::rpc::HighestCommittedBlockResponse, HttpErrorResponse> { 
        let raw_bytes = self
            .networking
            .get_response("highest_committed_block")
            .await
            .map_err(PChainClientError::new)?; 

            pchain_types::rpc::HighestCommittedBlockResponse::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))
    }

    /// `transaction` sends request to get transaction by specified tx hash.
    pub async fn transaction(
        &self, 
        request: &pchain_types::rpc::TransactionRequest
    ) -> Result<pchain_types::rpc::TransactionResponseV1, HttpErrorResponse> { 
        let data = pchain_types::rpc::TransactionRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("transaction", data)
            .await
            .map_err(PChainClientError::new)?; 

        let response: pchain_types::rpc::TransactionResponseV1 = pchain_types::rpc::TransactionResponseV1::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }

    /// `receipt` sends request to get receipt with transaction, block hash and position by specified tx hash.
    pub async fn receipt(
        &self, 
        request: &pchain_types::rpc::ReceiptRequest
    ) -> Result<pchain_types::rpc::ReceiptResponseV1, HttpErrorResponse> { 
        let data = pchain_types::rpc::ReceiptRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("receipt", data)
            .await
            .map_err(PChainClientError::new)?; 

        let response: pchain_types::rpc::ReceiptResponseV1 = pchain_types::rpc::ReceiptResponseV1::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }

    /// `transaction_position` sends request to get transaction position in block by specified tx hash.
    pub async fn transaction_position(
        &self, 
        request: &pchain_types::rpc::TransactionPositionRequest
    ) -> Result<pchain_types::rpc::TransactionPositionResponse, HttpErrorResponse> { 
        let data = pchain_types::rpc::TransactionPositionRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("transaction_position", data)
            .await
            .map_err(PChainClientError::new)?; 

        let response: pchain_types::rpc::TransactionPositionResponse = pchain_types::rpc::TransactionPositionResponse::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }
}