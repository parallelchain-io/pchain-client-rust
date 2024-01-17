/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

use async_trait::async_trait;
use pchain_types::rpc::TransactionV1OrV2;
use pchain_types::serialization::{Serializable, Deserializable};
use crate::error::{ self as PChainClientError, HttpErrorResponse };
use crate::networking::{Networking, NetworkProvider};

/// Client with methods corresponding to fullnode RPC V2.
pub struct ClientV2 {
    /// `networking` denotes instance of reqwest::Client.
    networking: Networking,
}

#[async_trait]
impl NetworkProvider for ClientV2 {
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

impl ClientV2 {
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
        tx: &TransactionV1OrV2
    ) -> Result<pchain_types::rpc::SubmitTransactionResponseV2, HttpErrorResponse> { 
        let request = pchain_types::rpc::SubmitTransactionRequestV2{ transaction: tx.clone() };
        let data = pchain_types::rpc::SubmitTransactionRequestV2::serialize(&request); 

        let raw_bytes = self
            .networking
            .post_response("submit_transaction/v2", data)
            .await
            .map_err(PChainClientError::new)?; 

        let response: pchain_types::rpc::SubmitTransactionResponseV2 = pchain_types::rpc::SubmitTransactionResponseV2::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;  
        
        Ok(response)
    }

    /// `state` sends request to query account data from world state.
    pub async fn state(
        &self, 
        request: &pchain_types::rpc::StateRequest
    ) -> Result<pchain_types::rpc::StateResponseV2, HttpErrorResponse> { 
        let data = pchain_types::rpc::StateRequest::serialize(request);  
        
        let raw_bytes = self
            .networking
            .post_response("state/v2", data)
            .await
            .map_err(PChainClientError::new)?; 

        let state_response: pchain_types::rpc::StateResponseV2 = pchain_types::rpc::StateResponseV2::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(state_response)
    }

    /// `view` sends request to execute a contract view call.
    pub async fn view(
        &self, 
        request: &pchain_types::rpc::ViewRequest
    ) -> Result<pchain_types::rpc::ViewResponseV2, HttpErrorResponse> { 
        let data = pchain_types::rpc::ViewRequest::serialize(request);  
        
        let raw_bytes = self
            .networking
            .post_response("view/v2", data)
            .await
            .map_err(PChainClientError::new)?; 

        let state_response: pchain_types::rpc::ViewResponseV2 = pchain_types::rpc::ViewResponseV2::deserialize(&raw_bytes)
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
    ) -> Result<pchain_types::rpc::BlockResponseV2, HttpErrorResponse> { 
        let data = pchain_types::rpc::BlockRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("block/v2", data)
            .await
            .map_err(PChainClientError::new)?; 

        let response: pchain_types::rpc::BlockResponseV2 = pchain_types::rpc::BlockResponseV2::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }

    /// `block_header` sends request to get block header starting from specified block hash.
    pub async fn block_header(
        &self, 
        request: &pchain_types::rpc::BlockHeaderRequest
    ) -> Result<pchain_types::rpc::BlockHeaderResponseV2, HttpErrorResponse> { 
        let data = pchain_types::rpc::BlockHeaderRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("block_header/v2", data)
            .await
            .map_err(PChainClientError::new)?; 

        let response: pchain_types::rpc::BlockHeaderResponseV2 = pchain_types::rpc::BlockHeaderResponseV2::deserialize(&raw_bytes)
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
    ) -> Result<pchain_types::rpc::TransactionResponseV2, HttpErrorResponse> { 
        let data = pchain_types::rpc::TransactionRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("transaction/v2", data)
            .await
            .map_err(PChainClientError::new)?; 

        let response: pchain_types::rpc::TransactionResponseV2 = pchain_types::rpc::TransactionResponseV2::deserialize(&raw_bytes)
        .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(response)
    }

    /// `receipt` sends request to get receipt with transaction, block hash and position by specified tx hash.
    pub async fn receipt(
        &self, 
        request: &pchain_types::rpc::ReceiptRequest
    ) -> Result<pchain_types::rpc::ReceiptResponseV2, HttpErrorResponse> { 
        let data = pchain_types::rpc::ReceiptRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("receipt/v2", data)
            .await
            .map_err(PChainClientError::new)?; 

        let response: pchain_types::rpc::ReceiptResponseV2 = pchain_types::rpc::ReceiptResponseV2::deserialize(&raw_bytes)
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

    /// `subscribe_to_transaction_events` sends request to register a websocket service for transaction events notification.
    ///  Returns client id string for websocket connection.
    pub async fn subscribe_to_transaction_events(
        &self, 
        request: &pchain_types::rpc::SubscribeToTransactionEventsRequest
    ) -> Result<String, HttpErrorResponse> { 
        let data = pchain_types::rpc::SubscribeToTransactionEventsRequest::serialize(request);  

        let raw_bytes = self
            .networking
            .post_response("subscribe_to_transaction_events", data)
            .await
            .map_err(PChainClientError::new)?; 

        let uuid = Deserializable::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;    

        Ok(uuid)
    }
}