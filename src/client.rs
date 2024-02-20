/*
    Copyright © 2023, ParallelChain Lab
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Functions to communicate with ParallelChain RPC endpoints. Users are required to provide
//! corresponding [request](pchain_types::rpc) specified in pchain_types in order to get a
//! correct response.

use async_trait::async_trait;
use pchain_types::{
    blockchain::TransactionV1,
    rpc::{
        BlockHashByHeightRequest, BlockHashByHeightResponse, BlockHeaderRequest,
        BlockHeaderResponseV1, BlockHeaderResponseV2, BlockHeightByHashRequest,
        BlockHeightByHashResponse, BlockRequest, BlockResponseV1, BlockResponseV2, DepositsRequest,
        DepositsResponse, HighestCommittedBlockResponse, PoolsRequest, PoolsResponse,
        ReceiptRequest, ReceiptResponseV1, ReceiptResponseV2, StakesRequest, StakesResponse,
        StateRequest, StateResponseV1, StateResponseV2, SubmitTransactionRequestV1,
        SubmitTransactionRequestV2, SubmitTransactionResponseV1, SubmitTransactionResponseV2,
        TransactionPositionRequest, TransactionPositionResponse, TransactionRequest,
        TransactionResponseV1, TransactionResponseV2, TransactionV1OrV2, ValidatorSetsRequest,
        ValidatorSetsResponse, ViewRequest, ViewResponseV1, ViewResponseV2,
    },
    serialization::{Deserializable, Serializable},
};

use crate::{
    error::{self as PChainClientError, HttpErrorResponse},
    networking::{NetworkProvider, Networking},
};

/// [Client] sets up the networking with methods corresponding to both fullnode RPC V1 and
/// fullnode RPC V2.
pub struct Client {
    /// `networking` denotes instance of reqwest::Client.
    networking: Networking,
}

#[async_trait]
impl NetworkProvider for Client {
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

impl Client {
    /// `new` creates a new instance of a pchain_client given a network provider.
    /// # Arguments
    /// * `rpc_base_url` - base URL of Parallelchain RPC endpoints
    ///
    pub fn new(rpc_base_url: &str) -> Self {
        Self {
            networking: Networking::new(String::from(rpc_base_url)),
        }
    }

    /// `validator_sets` sends request to query previous / current / next validator and delegator sets.
    pub async fn validator_sets(
        &self,
        request: &ValidatorSetsRequest,
    ) -> Result<ValidatorSetsResponse, HttpErrorResponse> {
        let data = ValidatorSetsRequest::serialize(request);

        let raw_bytes = self
            .networking
            .post_response("validator_sets", data)
            .await
            .map_err(PChainClientError::new)?;

        let response = ValidatorSetsResponse::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;

        Ok(response)
    }

    /// `pools` sends request to query pools with a set of operator address, with or without stakes of each pool.
    pub async fn pools(&self, request: &PoolsRequest) -> Result<PoolsResponse, HttpErrorResponse> {
        let data = PoolsRequest::serialize(request);

        let raw_bytes = self
            .networking
            .post_response("pools", data)
            .await
            .map_err(PChainClientError::new)?;

        let response = PoolsResponse::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;

        Ok(response)
    }

    /// `stakes` sends request to query stakes with a set of (operator address, owner address).
    pub async fn stakes(
        &self,
        request: &StakesRequest,
    ) -> Result<StakesResponse, HttpErrorResponse> {
        let data = StakesRequest::serialize(request);

        let raw_bytes = self
            .networking
            .post_response("stakes", data)
            .await
            .map_err(PChainClientError::new)?;

        let response = StakesResponse::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;

        Ok(response)
    }

    /// `deposits` sends request to query deposits with a set of (operator address, owner address).
    pub async fn deposits(
        &self,
        request: &DepositsRequest,
    ) -> Result<DepositsResponse, HttpErrorResponse> {
        let data = DepositsRequest::serialize(request);

        let raw_bytes = self
            .networking
            .post_response("deposits", data)
            .await
            .map_err(PChainClientError::new)?;

        let response = DepositsResponse::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;

        Ok(response)
    }

    /// `block_height_by_hash` sends request to get block height by specified block hash.
    pub async fn block_height_by_hash(
        &self,
        request: &BlockHeightByHashRequest,
    ) -> Result<BlockHeightByHashResponse, HttpErrorResponse> {
        let data = BlockHeightByHashRequest::serialize(request);

        let raw_bytes = self
            .networking
            .post_response("block_height_by_hash", data)
            .await
            .map_err(PChainClientError::new)?;

        let response = BlockHeightByHashResponse::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;

        Ok(response)
    }

    /// `block_hash_by_height` sends request to get block hash by specified block height.
    pub async fn block_hash_by_height(
        &self,
        request: &BlockHashByHeightRequest,
    ) -> Result<BlockHashByHeightResponse, HttpErrorResponse> {
        let data = BlockHashByHeightRequest::serialize(request);

        let raw_bytes = self
            .networking
            .post_response("block_hash_by_height", data)
            .await
            .map_err(PChainClientError::new)?;

        let response = BlockHashByHeightResponse::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;

        Ok(response)
    }

    /// `highest_committed_block` sends request to get the latest block on ParallelChain.
    pub async fn highest_committed_block(
        &self,
    ) -> Result<HighestCommittedBlockResponse, HttpErrorResponse> {
        let raw_bytes = self
            .networking
            .get_response("highest_committed_block")
            .await
            .map_err(PChainClientError::new)?;

        HighestCommittedBlockResponse::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))
    }

    /// `transaction_position` sends request to get transaction position in block by specified tx hash.
    pub async fn transaction_position(
        &self,
        request: &TransactionPositionRequest,
    ) -> Result<TransactionPositionResponse, HttpErrorResponse> {
        let data = TransactionPositionRequest::serialize(request);

        let raw_bytes = self
            .networking
            .post_response("transaction_position", data)
            .await
            .map_err(PChainClientError::new)?;

        let response = TransactionPositionResponse::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;

        Ok(response)
    }

    /// `submit_transaction_v1` sends request to submit a transaction using V1 RPC.
    pub async fn submit_transaction_v1(
        &self,
        tx: &TransactionV1,
    ) -> Result<SubmitTransactionResponseV1, HttpErrorResponse> {
        let request = SubmitTransactionRequestV1 {
            transaction: tx.clone(),
        };
        let data = SubmitTransactionRequestV1::serialize(&request);

        let raw_bytes = self
            .networking
            .post_response("submit_transaction", data)
            .await
            .map_err(PChainClientError::new)?;

        let response = SubmitTransactionResponseV1::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;

        Ok(response)
    }

    /// `submit_transaction_v2` sends request to submit a transaction using V2 RPC.
    pub async fn submit_transaction_v2(
        &self,
        tx: &TransactionV1OrV2,
    ) -> Result<SubmitTransactionResponseV2, HttpErrorResponse> {
        let request = SubmitTransactionRequestV2 {
            transaction: tx.clone(),
        };
        let data = SubmitTransactionRequestV2::serialize(&request);

        let raw_bytes = self
            .networking
            .post_response("submit_transaction/v2", data)
            .await
            .map_err(PChainClientError::new)?;

        let response = SubmitTransactionResponseV2::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;

        Ok(response)
    }

    /// `state_v1` sends request to query account data from world state using V1 RPC.
    pub async fn state_v1(
        &self,
        request: &StateRequest,
    ) -> Result<StateResponseV1, HttpErrorResponse> {
        let data = StateRequest::serialize(request);

        let raw_bytes = self
            .networking
            .post_response("state", data)
            .await
            .map_err(PChainClientError::new)?;

        let state_response = StateResponseV1::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;

        Ok(state_response)
    }

    /// `state_v2` sends request to query account data from world state using V2 RPC.
    pub async fn state_v2(
        &self,
        request: &StateRequest,
    ) -> Result<StateResponseV2, HttpErrorResponse> {
        let data = StateRequest::serialize(request);

        let raw_bytes = self
            .networking
            .post_response("state/v2", data)
            .await
            .map_err(PChainClientError::new)?;

        let state_response = StateResponseV2::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;

        Ok(state_response)
    }

    /// `view_v1` sends request to execute a contract view call using V1 RPC.
    pub async fn view_v1(
        &self,
        request: &ViewRequest,
    ) -> Result<ViewResponseV1, HttpErrorResponse> {
        let data = ViewRequest::serialize(request);

        let raw_bytes = self
            .networking
            .post_response("view", data)
            .await
            .map_err(PChainClientError::new)?;

        let state_response = ViewResponseV1::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;

        Ok(state_response)
    }

    /// `view_v2` sends request to execute a contract view call using V2 RPC.
    pub async fn view_v2(
        &self,
        request: &ViewRequest,
    ) -> Result<ViewResponseV2, HttpErrorResponse> {
        let data = ViewRequest::serialize(request);

        let raw_bytes = self
            .networking
            .post_response("view/v2", data)
            .await
            .map_err(PChainClientError::new)?;

        let state_response = ViewResponseV2::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;

        Ok(state_response)
    }

    /// `block_v1` sends request to get full block data starting from specified block hash using V1 RPC.
    pub async fn block_v1(
        &self,
        request: &BlockRequest,
    ) -> Result<BlockResponseV1, HttpErrorResponse> {
        let data = BlockRequest::serialize(request);

        let raw_bytes = self
            .networking
            .post_response("block", data)
            .await
            .map_err(PChainClientError::new)?;

        let response = BlockResponseV1::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;

        Ok(response)
    }

    /// `block_v2` sends request to get full block data starting from specified block hash using V2 RPC.
    pub async fn block_v2(
        &self,
        request: &BlockRequest,
    ) -> Result<BlockResponseV2, HttpErrorResponse> {
        let data = BlockRequest::serialize(request);

        let raw_bytes = self
            .networking
            .post_response("block/v2", data)
            .await
            .map_err(PChainClientError::new)?;

        let response = BlockResponseV2::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;

        Ok(response)
    }

    /// `block_header_v1` sends request to get block header starting from specified block hash using V1 RPC.
    pub async fn block_header_v1(
        &self,
        request: &BlockHeaderRequest,
    ) -> Result<BlockHeaderResponseV1, HttpErrorResponse> {
        let data = BlockHeaderRequest::serialize(request);

        let raw_bytes = self
            .networking
            .post_response("block_header", data)
            .await
            .map_err(PChainClientError::new)?;

        let response = BlockHeaderResponseV1::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;

        Ok(response)
    }

    /// `block_header_v2` sends request to get block header starting from specified block hash using V2 RPC.
    pub async fn block_header_v2(
        &self,
        request: &BlockHeaderRequest,
    ) -> Result<BlockHeaderResponseV2, HttpErrorResponse> {
        let data = BlockHeaderRequest::serialize(request);

        let raw_bytes = self
            .networking
            .post_response("block_header/v2", data)
            .await
            .map_err(PChainClientError::new)?;

        let response = BlockHeaderResponseV2::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;

        Ok(response)
    }

    /// `transaction_v1` sends request to get transaction by specified tx hash using V1 RPC.
    pub async fn transaction_v1(
        &self,
        request: &TransactionRequest,
    ) -> Result<TransactionResponseV1, HttpErrorResponse> {
        let data = TransactionRequest::serialize(request);

        let raw_bytes = self
            .networking
            .post_response("transaction", data)
            .await
            .map_err(PChainClientError::new)?;

        let response = TransactionResponseV1::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;

        Ok(response)
    }

    /// `transaction_v2` sends request to get transaction by specified tx hash using V2 RPC.
    pub async fn transaction_v2(
        &self,
        request: &TransactionRequest,
    ) -> Result<TransactionResponseV2, HttpErrorResponse> {
        let data = TransactionRequest::serialize(request);

        let raw_bytes = self
            .networking
            .post_response("transaction/v2", data)
            .await
            .map_err(PChainClientError::new)?;

        let response = TransactionResponseV2::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;

        Ok(response)
    }

    /// `receipt_v1` sends request to get receipt with transaction, block hash and position by specified tx hash using V1 RPC.
    pub async fn receipt_v1(
        &self,
        request: &ReceiptRequest,
    ) -> Result<ReceiptResponseV1, HttpErrorResponse> {
        let data = ReceiptRequest::serialize(request);

        let raw_bytes = self
            .networking
            .post_response("receipt", data)
            .await
            .map_err(PChainClientError::new)?;

        let response = ReceiptResponseV1::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;

        Ok(response)
    }

    /// `receipt_v2` sends request to get receipt with transaction, block hash and position by specified tx hash using V2 RPC.
    pub async fn receipt_v2(
        &self,
        request: &ReceiptRequest,
    ) -> Result<ReceiptResponseV2, HttpErrorResponse> {
        let data = ReceiptRequest::serialize(request);

        let raw_bytes = self
            .networking
            .post_response("receipt/v2", data)
            .await
            .map_err(PChainClientError::new)?;

        let response = ReceiptResponseV2::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;

        Ok(response)
    }
}
