
/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Implementation of utility methods related to hashing, signing messages and generating keypairs.

use ed25519_dalek::Signer;
use pchain_types::Base64URL;
use sha2::{Sha256, Digest};

use crate::error::ClientError;

pub type DataHash = [u8; DATA_HASH_LEN];
pub const DATA_HASH_LEN: usize = 32;
pub const ADDRESS_LENGTH: usize = 32;
pub const KEYPAIR_LENGTH: usize = 64;
pub const PRIVATEKEY_LENGTH: usize = 32;

/// `sign` returns cipherext cryptographically signed with exising keypair.
/// # Arguments
/// * `message` - message of type:string to be signed
/// 
pub fn sign(keypair: &str, message: &str) -> Result<String, String> {
    let keypair = {
        let keypair_bs = match Base64URL::decode(&keypair) {
            Ok(kp) => kp,
            Err(_) => return Err(ClientError::FailToBase64DecodeKeypair.into())
        };
        match ed25519_dalek::Keypair::from_bytes(&keypair_bs) {
            Ok(kp) => kp,
            Err(_) => return Err(ClientError::InvalidED25519Keypair.into())
        }
    };

    let serialized_credentials = match Base64URL::decode(&message){
        Ok(m) => m,
        Err(_) => return Err(ClientError::InvalidBase64Encoding(String::from("message")).into()),
    };
    let ciphertext : ed25519_dalek::Signature = keypair.sign(&serialized_credentials[..]);
    let ciphertext = Base64URL::encode(ciphertext);

    Ok(ciphertext.to_string())
}

/// `compute_contract_address` computes address of a contract from sender address and nonce.
/// # Arguments
/// * `sender_address` - public address of the deployer of the contract
/// * `nonce` - current nonce of the owner account on ParallelChain
/// 
pub fn compute_contract_address(
    sender_address: &pchain_types::PublicAddress, 
    nonce: u64
) -> Vec<u8> {
    let mut hasher = Sha256::new();
    let mut pre_image = Vec::new();
    pre_image.extend(sender_address);
    pre_image.extend(nonce.to_le_bytes().to_vec());

    hasher.update(pre_image);

    hasher.finalize().to_vec()
}
