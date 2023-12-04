/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! HTTP networking with specific fullnodes.

/// [Networking] houses all HTTP methods used by the Client 
/// to serve responses to the user.
///     
use async_trait::async_trait;

pub(crate) struct Networking {
    /// `provider` denotes current ParallelChain Fullnode Provider URL.
    provider: String,
    /// `client` denotes an instance of reqwest::Client for serving HTTP requests.
    pub client: reqwest::Client,
}

impl Networking {
    /// `new` creates a new network provider for Client. Panics if failed to initialized TLS backend connection.
    /// 
    ///  Timeout setting for the Client:
    ///     - connect : 10 secs
    ///     - read and write operations: 30 secs
    /// 
    pub fn new(provider: String) -> Self {
        Networking { 
            provider,
            client: reqwest::Client::builder().connect_timeout(std::time::Duration::from_secs(10)).build().expect("TLS backend cannot be initialized")
        }
    }

    /// `set_provider` sets a new network provider for Client.
    /// # Arguments
    /// * `url` - ParallelChain RPC base network URL
    /// 
    pub fn set_provider(&mut self, url: &str) {
        self.provider = url.to_string();
    }

    /// `get_provider` get current network provider base url.
    /// 
    pub fn get_provider(&self) -> String {
        self.provider.clone()
    }

    /// `is_provider_up` sends a GET request to the network provider to check if 
    /// the current provider is up. 
    /// 
    pub async fn is_provider_up(&self) -> bool {
        let response = self.get_response("").await;

        response.is_ok()
    }
    
    /// `post_request` sends a POST request to the network provider for Client.
    /// # Arguments
    /// * `request_url` - The request URL 
    /// * `body` - serialized pchain_types::SignedTx
    /// 
    pub async fn post_request(&self, request_url: &str, body: Vec<u8>) -> Result<reqwest::Response, reqwest::Error> {
        self.client.post(request_url).body(body).send().await
    }

    /// `post_response` is a helper to return server side responses from HTTP `POST methods` defined in this namespace.
    /// # Arguments
    /// * `request_url` - The request URL
    /// * `data` - Vector of bytes serialized from generic types
    /// 
    pub async fn post_response(&self, request_url: &str, data: Vec<u8>) -> Result<bytes::Bytes, String> {
        let url = format!("{}/{}", &self.provider, request_url);
        
        let response = self
            .post_request(&url, data)
            .await
            .map_err(|e| e.to_string())?; 
                
        match response.status() {
            reqwest::StatusCode::OK => {
                Ok(response
                        .bytes().await.map_err(|e| e.to_string())?)
            },
            _ => {
                Err(response.text().await.map_err(|e| e.to_string())?)
            }
        }
    }

    /// `get_request` sends a GET request to the network provider.
    /// # Arguments
    /// * `request_url` - The request URL 
    /// 
    pub async fn get_request(&self, request_url: &str) -> Result<reqwest::Response, reqwest::Error> {
        self.client.get(request_url).send().await
    }

    // `get_response` is a helper to return server side responses from HTTP `GET methods` defined in this namespace.
    // # Arguments
    // * `request_url` - The request URL 
    // 
    pub async fn get_response(&self, request_url: &str) -> Result<bytes::Bytes, String> {
        let url = format!("{}/{}", &self.provider, request_url);
        let response = self
            .get_request(&url)
            .await
            .map_err(|e| e.to_string())?; 

        match response.status() {
            reqwest::StatusCode::OK => {
                Ok(response
                        .bytes().await.map_err(|e| e.to_string())?)
            },
            _ => {
                Err(response.text().await.map_err(|e| e.to_string())?)
            }
        }
    }
}

/// A trait used for Parallelchain RPC API provider setup.
#[async_trait]
pub trait NetworkProvider {
    /// assigns new network provider for Client.
    /// 
    /// # Arguments
    /// * `rpc_base_url` - base URL of Parallelchain RPC endpoints
    fn set_provider(&mut self, rpc_base_url: &str);

    /// get current network provider base url.
    fn get_provider(&self) -> String;

    /// check if the current provider is up. 
    /// 
    /// # Return
    /// true if server is up, otherwise returns false.
    async fn is_provider_up(&self) -> bool;
}