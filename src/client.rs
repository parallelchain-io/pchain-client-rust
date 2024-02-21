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
    /// `networking` denotes the instance of reqwest::Client.
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

    /// `highest_committed_block` sends a request to get the latest block on ParallelChain.
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

    /// `post_request` defines the generic implementation of POST requests for RPC:
    /// 1. serialize the input request.
    /// 2. send a POST request to the network provider for the Client.
    /// 3. deserialize the output response.
    async fn post_request<I: Serializable, O: Deserializable>(
        &self,
        input: &I,
        endpoint_path: &str,
    ) -> Result<O, HttpErrorResponse> {
        let data = <I as Serializable>::serialize(input);

        let raw_bytes = self
            .networking
            .post_response(endpoint_path, data)
            .await
            .map_err(PChainClientError::new)?;

        let response = <O as Deserializable>::deserialize(&raw_bytes)
            .map_err(|e| PChainClientError::new(e.to_string()))?;
        Ok(response)
    }

    /// `validator_sets` sends a request to query previous / current / next validator
    /// and delegator sets.
    pub async fn validator_sets(
        &self,
        request: &ValidatorSetsRequest,
    ) -> Result<ValidatorSetsResponse, HttpErrorResponse> {
        self.post_request(request, "validator_sets").await
    }

    /// `pools` sends a request to query pools with a set of operator addresses, with or
    /// without stakes of each pool.
    pub async fn pools(&self, request: &PoolsRequest) -> Result<PoolsResponse, HttpErrorResponse> {
        self.post_request(request, "pools").await
    }

    /// `stakes` sends a request to query stakes with a set of
    /// (operator address, owner address).
    pub async fn stakes(
        &self,
        request: &StakesRequest,
    ) -> Result<StakesResponse, HttpErrorResponse> {
        self.post_request(request, "stakes").await
    }

    /// `deposits` sends a request to query deposits with a set of
    /// (operator address, owner address).
    pub async fn deposits(
        &self,
        request: &DepositsRequest,
    ) -> Result<DepositsResponse, HttpErrorResponse> {
        self.post_request(request, "deposits").await
    }

    /// `block_height_by_hash` sends a request to get block height by specified block
    /// hash.
    pub async fn block_height_by_hash(
        &self,
        request: &BlockHeightByHashRequest,
    ) -> Result<BlockHeightByHashResponse, HttpErrorResponse> {
        self.post_request(request, "block_height_by_hash").await
    }

    /// `block_hash_by_height` sends a request to get block hash by specified block
    ///  height.
    pub async fn block_hash_by_height(
        &self,
        request: &BlockHashByHeightRequest,
    ) -> Result<BlockHashByHeightResponse, HttpErrorResponse> {
        self.post_request(request, "block_hash_by_height").await
    }

    /// `transaction_position` sends a request to get transaction position in block by
    /// specified tx hash.
    pub async fn transaction_position(
        &self,
        request: &TransactionPositionRequest,
    ) -> Result<TransactionPositionResponse, HttpErrorResponse> {
        self.post_request(request, "transaction_position").await
    }

    /// `submit_transaction_v1` sends a request to submit a transaction using V1 RPC.
    pub async fn submit_transaction_v1(
        &self,
        tx: &TransactionV1,
    ) -> Result<SubmitTransactionResponseV1, HttpErrorResponse> {
        let request = SubmitTransactionRequestV1 {
            transaction: tx.clone(),
        };

        self.post_request(&request, "submit_transaction").await
    }

    /// `submit_transaction_v2` sends a request to submit a transaction using V2 RPC.
    pub async fn submit_transaction_v2(
        &self,
        tx: &TransactionV1OrV2,
    ) -> Result<SubmitTransactionResponseV2, HttpErrorResponse> {
        let request = SubmitTransactionRequestV2 {
            transaction: tx.clone(),
        };

        self.post_request(&request, "submit_transaction/v2").await
    }

    /// `state_v1` sends a request to query account data from world state using V1 RPC.
    pub async fn state_v1(
        &self,
        request: &StateRequest,
    ) -> Result<StateResponseV1, HttpErrorResponse> {
        self.post_request(request, "state").await
    }

    /// `state_v2` sends a request to query account data from world state using V2 RPC.
    pub async fn state_v2(
        &self,
        request: &StateRequest,
    ) -> Result<StateResponseV2, HttpErrorResponse> {
        self.post_request(request, "state/v2").await
    }

    /// `view_v1` sends a request to execute a contract view call using V1 RPC.
    pub async fn view_v1(
        &self,
        request: &ViewRequest,
    ) -> Result<ViewResponseV1, HttpErrorResponse> {
        self.post_request(request, "view").await
    }

    /// `view_v2` sends a request to execute a contract view call using V2 RPC.
    pub async fn view_v2(
        &self,
        request: &ViewRequest,
    ) -> Result<ViewResponseV2, HttpErrorResponse> {
        self.post_request(request, "view/v2").await
    }

    /// `block_v1` sends a request to get full block data starting from specified
    /// block hash using V1 RPC.
    pub async fn block_v1(
        &self,
        request: &BlockRequest,
    ) -> Result<BlockResponseV1, HttpErrorResponse> {
        self.post_request(request, "block").await
    }

    /// `block_v2` sends a request to get full block data starting from specified
    /// block hash using V2 RPC.
    pub async fn block_v2(
        &self,
        request: &BlockRequest,
    ) -> Result<BlockResponseV2, HttpErrorResponse> {
        self.post_request(request, "block/v2").await
    }

    /// `block_header_v1` sends a request to get block header starting from specified
    /// block hash using V1 RPC.
    pub async fn block_header_v1(
        &self,
        request: &BlockHeaderRequest,
    ) -> Result<BlockHeaderResponseV1, HttpErrorResponse> {
        self.post_request(request, "block_header").await
    }

    /// `block_header_v2` sends a request to get block header starting from specified
    /// block hash using V2 RPC.
    pub async fn block_header_v2(
        &self,
        request: &BlockHeaderRequest,
    ) -> Result<BlockHeaderResponseV2, HttpErrorResponse> {
        self.post_request(request, "block_header/v2").await
    }

    /// `transaction_v1` sends a request to get transaction by specified tx hash using
    /// V1 RPC.
    pub async fn transaction_v1(
        &self,
        request: &TransactionRequest,
    ) -> Result<TransactionResponseV1, HttpErrorResponse> {
        self.post_request(request, "transaction").await
    }

    /// `transaction_v2` sends a request to get transaction by specified tx hash using
    /// V2 RPC.
    pub async fn transaction_v2(
        &self,
        request: &TransactionRequest,
    ) -> Result<TransactionResponseV2, HttpErrorResponse> {
        self.post_request(request, "transaction/v2").await
    }

    /// `receipt_v1` sends a request to get receipt with transaction, block hash and
    /// position by specified tx hash using V1 RPC.
    pub async fn receipt_v1(
        &self,
        request: &ReceiptRequest,
    ) -> Result<ReceiptResponseV1, HttpErrorResponse> {
        self.post_request(request, "receipt").await
    }

    /// `receipt_v2` sends a request to get receipt with transaction, block hash and
    /// position by specified tx hash using V2 RPC.
    pub async fn receipt_v2(
        &self,
        request: &ReceiptRequest,
    ) -> Result<ReceiptResponseV2, HttpErrorResponse> {
        self.post_request(request, "receipt/v2").await
    }
}
