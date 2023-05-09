/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Implementation of methods which perform error handling for this library.

/// HttpError transform the http error code (integer in data type string) received from standatd code to human readable string.
#[derive(Debug, Clone, Copy)]
pub struct HttpError();

impl HttpError {
    // `new` creates an ErrorResponse given an error code.
    // from RPC endpoint 
    // # Arguments 
    // * `error_code` - error code received from RPC endpoint.
    // 
    pub fn new(error_code: String) -> HttpErrorResponse {
        match error_code.parse::<i16>() {
            Ok(err) => Self::response(err as u16),
            Err(_) => {
                if error_code.contains("Connection refused"){
                    format!("Connection refused. Please check if the provider is live.")
                }
                else{
                    format!("Unknown error occured. {}", error_code)
                }
            }
        }
    }

    // `response` is a helper which receives an error code and sends 
    // back a human readable message to the user.
    // # Arguments 
    // * `error_code` - error code received from RPC endpoint.
    // 
    fn response(error_code: u16) -> HttpErrorResponse {
        match error_code {
            response_code::status400::INPUT_DECODE_FAILURE => String::from("Input query parameter is not a pchain_types::Base64URL encoded string."),
            response_code::status400::INCORRECT_URL_AND_QUERY_PARAMS => String::from("Incorrect url or query parameters."),
            response_code::status500::VIEW_SERVICE_CHANNEL_ERROR => String::from("Internal Server Error. Server busy and failed to handle new request."),
            response_code::status500::VIEW_SERVICE_REQUEST_TIMEOUT => String::from("Internal Server Error. Request Timeout"),
            _ => panic!("Irrecoverable Error. Unknown error code encountered. Please post an issue on ParallelChain Github Repository."),
        }
    }
}
/// `response_code` houses the current error codes defined in Fullnode.
pub(crate) mod response_code {
    pub(crate) mod status400 {
        pub(crate) const INPUT_DECODE_FAILURE: u16 = 0x44C;
        pub(crate) const INCORRECT_URL_AND_QUERY_PARAMS: u16 = 0x44E;
    }
    pub(crate) mod status500{
        pub(crate) const VIEW_SERVICE_CHANNEL_ERROR: u16 = 0x57D;
        pub(crate) const VIEW_SERVICE_REQUEST_TIMEOUT: u16 = 0x57E;
    }
}

pub type Errormsg = String;
pub type Identity = String;
pub enum ClientError {
    InvalidJson(serde_json::error::Error),
    MissingFieldinJson(String),
    FailToBase64DecodeKeypair,
    FailToConvertReturnDataToTargetType(std::io::Error),
    FailToSerializeCallArgument(Errormsg),
    InvalidED25519Keypair,
    InvalidBase64Encoding(Identity),
    IncorrectBase64urlLength,

}


// Error string contains key point of error as they are going to be further handled by pchain cli.
impl Into<String> for  ClientError {
    fn into(self) -> String {
        match self {
            ClientError::InvalidJson(e) => format!("Provided json is not valid. {e}"),
            ClientError::MissingFieldinJson(field_name) => format!("Provided json does not contain field with name : {field_name}"),
            ClientError::FailToBase64DecodeKeypair => String::from("Fail to base64 decode keypair."),
            ClientError::FailToConvertReturnDataToTargetType(e) => format!("Fail to convert to target data type. {:?}", e),
            ClientError::FailToSerializeCallArgument(e) => format!("Fail to serialize call argument. {:?}", e),
            ClientError::InvalidED25519Keypair => String::from("Fail to convert decoded keypair to ed25519::Keypair."),
            ClientError::InvalidBase64Encoding(identity) => format!("Provided {identity} has invalid base64 encoding"),
            ClientError::IncorrectBase64urlLength => String::from("Incorrect length. Correct length should be 32 bytes long."),

        }
    }
}



pub type HttpErrorResponse =  String;
