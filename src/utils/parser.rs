/*
    Copyright © 2023, ParallelChain Lab 
    Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
*/

//! Implementation of utility methods related to parsing pchain_types::CallData.

use std::{convert::TryInto, io::{ErrorKind, Error}};
use regex::Regex;
use serde_big_array::Array;
use serde_json::Value;
use borsh::{BorshSerialize, BorshDeserialize};
use crate::error::ClientError;

/// [CallArguments] defines data structures related to serde serializable/deserializable  
/// method arguments of pchain_types::Call Transaction Command.
#[derive(Debug)]
pub struct CallArguments {
    pub arguments: Vec<(String, String)> // type-value pair
}

impl CallArguments {
    // `from_json` Read from a string in json and deserialize it to vector of arg-type to arg-value pairs.
    // Throws error if decode fails.
    // # Arguments
    // * `json_data` - the json represented in string which contain call arguments data
    //
    pub fn from_json(json_data: &str) -> core::result::Result<CallArguments, String> {
        let json_val: Value = match serde_json::from_str(json_data) {
            Ok(val) => { val },
            Err(e) => return Err(ClientError::InvalidJson(e).into())
        };

        let json_args: Vec<Value> = match &json_val["arguments"].as_array() {
            Some(args) => { args.to_vec() },
            None => return Err(ClientError::MissingFieldinJson(String::from("arguments")).into()),
        };

        // parse arguments
        let arguments: Vec<(String, String)> = json_args.iter().filter_map(|jarg|{
            if let Some(j_type) = jarg["argument_type"].as_str() {
                if let Some(j_val) = jarg["argument_value"].as_str() {
                    Some((j_type.to_string(), j_val.to_string()))
                } else {None}
            } else {None}
        }).collect();
        
        Ok(CallArguments{ arguments })
    }
}

// `base64url_to_bytes32` decodes a Base64URL string into a slice of size 32. 
// Throws error if decode fails.
// # Arguments
// * `base64url` - the string argument which is to be decoded
//
pub fn base64url_to_bytes32(base64url: &str) -> Result<[u8; 32], String> {
    let value: [u8; 32] = match pchain_types::Base64URL::decode(&base64url){
        Ok(s) => match s.try_into() {
            Ok(s) => s,
            Err(_) => {
                return Err(ClientError::IncorrectBase64urlLength.into());
            }
        },
        Err(_) => {
            return Err(ClientError::InvalidBase64Encoding(String::from("string")).into());
        }
    };

    Ok(value)
}

// `call_result_to_data_type` deserialize call result to requested data type. 
// Throws error if decode fails.
//
// Accept data type: i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, bool, String, [u8;32], [u8;64],
// Vec<i8>, Vec<i16>, Vec<i32>, Vec<i64>, Vec<i128>, Vec<u8>, Vec<u16>, Vec<u32>, Vec<u64>, Vec<u128>, 
// Vec<bool>, Vec<String>.
//
// # Arguments
// * `vec` - the call result represented in vector
// * `data_type` - the call result corresponding data type, represneted in `String`
//
pub fn call_result_to_data_type(vec: &Vec<u8>, data_type: String) -> Result<String, String> {
    let dt_no_space: String = data_type.replace(" ", "");
    macro_rules! convert_to_data_type {
        ($d:expr, $($t:ty,)*) => {
            $(
                if dt_no_space == stringify!($t) {
                    match <$t>::deserialize(&mut vec.as_slice()) {
                        Ok(data) => {
                            return Ok(format!("{:?}", data));
                        },
                        Err(e) => {
                            return Err(ClientError::FailToConvertReturnDataToTargetType(e).into());
                        }
                    }
                }
            )*
        };
    }

    macro_rules! convert_to_vecs {
        ($d:expr, $($t:ty,)*) => {
            $(
                if dt_no_space == concat!("Vec<", stringify!($t), ">") {
                    match Vec::<$t>::deserialize(&mut vec.as_slice()) {
                        Ok(data) => {
                            return Ok(format!("{:?}", data));
                        },
                        Err(e) => {
                            return Err(ClientError::FailToConvertReturnDataToTargetType(e).into());
                        }
                    }
                }
            )*
        };
    }

    macro_rules! convert_to_slice {
        ($d:expr, $($s:expr,)*) => {
            $(
                if dt_no_space == concat!("[u8;", stringify!($s), "]") {
                    match <[u8; $s]>::deserialize(&mut vec.as_slice()) {
                        Ok(data) => {
                            return Ok(format!("{:?}", data));
                        },
                        Err(e) => {
                            return Err(ClientError::FailToConvertReturnDataToTargetType(e).into());
                        }
                    }
                }
            )*
            return Err(ClientError::FailToConvertReturnDataToTargetType(Error::new(ErrorKind::InvalidInput, "Provided data types is not supported")).into())

        };
    }

    convert_to_data_type!(data_type,
        u8, u16, u32, u64, u128,
        i8, i16, i32, i64, i128,
        bool, String,
    );

    convert_to_vecs!(data_type,
        u8, u16, u32, u64, u128,
        i8, i16, i32, i64, i128,
        bool, String,
    );

    convert_to_slice!(data_type,
        32, 64,
    );

}

// `serialize_call_arguments` serialize call arguments to bytes. 
// Throws error if decode fails.
//
// Accept data type: 
// accepted primitive types: i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, bool, String, ,
// Vector of accepted primitve types, Vector of Vector of accepted primitive types
// Option of accepted primitive types, Vector of option of accepted primitive types, Option of vector of accepted primitive types
// [u8;32], [u8;64]
//
// # Arguments
// * `value` - the call argument represented in string literal
// * `data_type` - the call argument corresponding data type, represneted in string literal
//
pub fn serialize_call_arguments(value: &str, data_type: &str) -> Result<Vec<u8>, String> {
    let mut dt_no_space: String = data_type.replace(" ", "");

    // if input type string is a slice of number type with length 32 or 64
    let re = Regex::new(r"^\[[ui](8|16|32|64|128);(32|64)]$").unwrap();
    if re.is_match(&dt_no_space) {
        // turn slice into serde json big array type
        dt_no_space = dt_no_space.replace("[", "Array<").replace(";", ",").replace("]", ">");
    }

    macro_rules! serialize_call_args {
        ($($t:ty,)*) => {
            $(
                if dt_no_space == stringify!($t).replace(" ", "") {                    
                    let data: $t = match serde_json::from_str(&value){
                        Ok(d) => d,
                        Err(e) => return Err(ClientError::FailToSerializeCallArgument(e.to_string()).into()),
                    };

                    match data.try_to_vec() {
                        Ok(data) => {
                            return Ok(data);
                        },
                        Err(e) => {
                            return Err(ClientError::FailToSerializeCallArgument(e.to_string()).into());
                        }
                    }
                }
            )*
            return Err(ClientError::FailToConvertReturnDataToTargetType(Error::new(ErrorKind::InvalidInput, "Provided data types is not supported")).into())
        };
    }

    serialize_call_args!(
        i8, i16, i32, i64, i128,
        u8, u16, u32, u64, u128,
        bool, String,

        Vec<i8>, Vec<i16>, Vec<i32>, Vec<i64>, Vec<i128>,
        Vec<u8>, Vec<u16>, Vec<u32>, Vec<u64>, Vec<u128>,
        Vec<bool>, Vec<String>, 

        Option<i8>, Option<i16>, Option<i32>, Option<i64>, Option<i128>,
        Option<u8>, Option<u16>, Option<u32>, Option<u64>, Option<u128>,
        Option<bool>, Option<String>,
        
        Vec<Vec<i8>>, Vec<Vec<i16>>, Vec<Vec<i32>>, Vec<Vec<i64>>, Vec<Vec<i128>>,
        Vec<Vec<u8>>, Vec<Vec<u16>>, Vec<Vec<u32>>, Vec<Vec<u64>>, Vec<Vec<u128>>,
        Vec<Vec<bool>>, Vec<Vec<String>>, 

        Option<Vec<i8>>, Option<Vec<i16>>, Option<Vec<i32>>, Option<Vec<i64>>, Option<Vec<i128>>,
        Option<Vec<u8>>, Option<Vec<u16>>, Option<Vec<u32>>, Option<Vec<u64>>, Option<Vec<u128>>,
        Option<Vec<bool>>, Option<Vec<String>>, 

        Vec<Option<i8>>, Vec<Option<i16>>, Vec<Option<i32>>, Vec<Option<i64>>, Vec<Option<i128>>,
        Vec<Option<u8>>, Vec<Option<u16>>, Vec<Option<u32>>, Vec<Option<u64>>, Vec<Option<u128>>,
        Vec<Option<bool>>, Vec<Option<String>>,

        Array<u8, 32>, Array<u8, 64>,
    );
}


#[cfg(test)]
mod test {
    use std::io::{ErrorKind, Error};

    use borsh::BorshSerialize;
    use crate::CallArguments;
    use crate::error::ClientError;
    use crate::parser::serialize_call_arguments;

    #[test]
    fn test_serialize_call_arrguments(){
        assert_eq!(serialize_call_arguments("[[[true]]]", "Vec<Vec<Vec<bool>>>"),
            Err(ClientError::FailToConvertReturnDataToTargetType(Error::new(ErrorKind::InvalidInput, "Provided data types is not supported")).into())
        );

        assert_eq!(serialize_call_arguments("false", "bool"), {
            let v = false;
            v.try_to_vec().map_err(|_| String::new())
        });

            assert_eq!(serialize_call_arguments("[false]", "Vec<bool>"), {
            let v: Vec<bool> = vec![false];
            v.try_to_vec().map_err(|_| String::new())
        });
        
        assert_eq!(serialize_call_arguments("[-2, -3, 5, 6]", "Vec< i32 >"), {
            let v: Vec<i32> = vec![-2, -3, 5, 6];
            v.try_to_vec().map_err(|_| String::new())
        });
        
        assert_eq!(serialize_call_arguments("[2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2]", "[ u8 ; 64 ]"), {
            let v: [u8; 64] = [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2];
            v.try_to_vec().map_err(|_| String::new())
        });


        assert_eq!(serialize_call_arguments("\"Fa\\\"l\\\"se\"", "String"), {
            let v = r#"Fa"l"se"#;
            v.try_to_vec().map_err(|_| String::new())
        });

        assert_eq!(serialize_call_arguments("\"Fa\\\"l\\\"se\"", "String"), {
            let v = "Fa\"l\"se";
            v.try_to_vec().map_err(|_| String::new())
        });

        assert_eq!(serialize_call_arguments("[\"sss\", \"ddd\"]", "Vec< String >"), {
            let v: Vec<String> = vec![String::from("sss"), String::from("ddd")];
            v.try_to_vec().map_err(|_| String::new())
        });

        assert_eq!(serialize_call_arguments("[\"sss\", \"ddd\"]", "Vec< Option < String > >"), {
            let v: Vec<Option<String>> = vec![Some(String::from("sss")), Some(String::from("ddd"))];
            v.try_to_vec().map_err(|_| String::new())
        });

        assert_eq!(serialize_call_arguments("[\"sss\", null]", "Vec< Option < String > >"), {
            let v: Vec<Option<String>> = vec![Some(String::from("sss")), None];
            v.try_to_vec().map_err(|_| String::new())
        });

        assert_eq!(serialize_call_arguments("[\"sss\", \"ddd\"]", "Option< Vec < String > >"), {
            let v: Option<Vec<String>> = Some(vec![String::from("sss"), String::from("ddd")]);
            v.try_to_vec().map_err(|_| String::new())
        });

        assert_eq!(serialize_call_arguments("null", "Option< Vec < String > >"), {
            let v: Option<Vec<String>> = None;
            v.try_to_vec().map_err(|_| String::new())
        });

        assert_eq!(serialize_call_arguments("[[\"sss\"], [\"ddd\"]]", "Vec< Vec < String > >"), {
            let v: Vec<Vec<String>> = vec![vec![String::from("sss")], vec![String::from("ddd")]];
            v.try_to_vec().map_err(|_| String::new())
        });
    }

    #[test]
    fn test_parse_arguments() {
        let json_string = r#" 
            {
                "arguments": [
                    {"argument_type": "i8", "argument_value":"-1"},
                    {"argument_type": "Vec<i8>", "argument_value":"[-1, -3, -5]"},
                    {"argument_type": "Vec<Vec<i8>>", "argument_value":"[[-1, -3], [-5]]"},
                    {"argument_type": "Vec<Opion<i8>>", "argument_value":"[-1, null, -5]"},
                    {"argument_type": "Opion<Vec<i8>>", "argument_value":"[-1, -3, -5]"},
                    {"argument_type": "Opion<Vec<i8>>", "argument_value":"null"},
            
                    {"argument_type": "i16", "argument_value":"-30000"},
                    {"argument_type": "Vec<i16>", "argument_value":"[-10001, -30001, -50001]"},
                    {"argument_type": "Vec<Vec<i16>>", "argument_value":"[[-10001, -30001], [-50001]]"},
                    {"argument_type": "Vec<Opion<i16>>", "argument_value":"[-10001, null, -50001]"},
                    {"argument_type": "Opion<Vec<i16>", "argument_value":"[-10001, -30001, -50001]"},
                    {"argument_type": "Opion<Vec<i16>>", "argument_value":"null"},
                    
                    {"argument_type": "i32", "argument_value":"-1094967295"},
                    {"argument_type": "Vec<i32>", "argument_value":"[-1,0,1]"},
            
                    {"argument_type": "i64", "argument_value":"-9046744073709551615"},
                    {"argument_type": "Vec<i64>", "argument_value":"[-1,0,1,65656565]"},
            
                    {"argument_type": "i128", "argument_value":"-9046744073709551615"},
                    {"argument_type": "Vec<i128>", "argument_value":"[-1,0,1,-65656565,0]"},
            
                    {"argument_type": "u8", "argument_value":"255"},
                    {"argument_type": "Vec<u8>", "argument_value":"[0]"},
            
                    {"argument_type": "u16", "argument_value":"65535"},
                    {"argument_type": "Vec<u16>", "argument_value":"[65535,6535]"},
            
                    {"argument_type": "u32", "argument_value":"4294967295"},
                    {"argument_type": "Vec<u32>", "argument_value":"[65535,6535,1919]"},
            
                    {"argument_type": "u64", "argument_value":"18446744073709551615"},
                    {"argument_type": "Vec<u64>", "argument_value":"[65535,6535,1919112123223]"},
            
                    {"argument_type": "u128", "argument_value":"18446744073709551616"},
                    {"argument_type": "Vec<u128>", "argument_value":"[65535,6535,1919112123223,123123,124124,125152]"},
            
                    {"argument_type": "bool", "argument_value": "true"},
                    {"argument_type": "Vec<bool>", "argument_value": "[true,false,true]"},
                    {"argument_type": "Vec<Vec<bool>>", "argument_value":"[[true, false], [true]]"},
                    {"argument_type": "Vec<Opion<bool>>", "argument_value":"[true,false,nul]"},
                    {"argument_type": "Opion<Vec<bool>", "argument_value":"[true,false,true]"},
                    {"argument_type": "Opion<Vec<bool>", "argument_value":"null"},
            
                    {"argument_type": "String", "argument_value": "string data"},
                    {"argument_type": "Vec<String>", "argument_value": "[\"string data\", \"asdaf\", \"1d1 as2\"]"},
                    {"argument_type": "Vec<Vec<String>>", "argument_value":"[[\"string data\"], [\"asdaf\", \"1d1 as2\"]]"},
                    {"argument_type": "Vec<Opion<String>>", "argument_value":"[\"string data\", null, \"1d1 as2\"]"},
                    {"argument_type": "Opion<Vec<String>", "argument_value":"[\"string data\", \"asdaf\", \"1d1 as2\"]"},
                    {"argument_type": "Opion<Vec<String>", "argument_value":"null"},

                    {"argument_type": "[u8, 32]", "argument_value": "[1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2]"},
                    {"argument_type": "[u8, 64]", "argument_value": "[1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2]"}
                ]
            }
            
        "#;

        let result = CallArguments::from_json(json_string);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().arguments.len(), 42);
    }

    #[test]
    fn test_callresult(){
        macro_rules! assert_data_types {
            ($($t:expr, $v:expr, $e:expr,)*) => {
                $(
                    let value = $v.try_to_vec().unwrap();

                    assert_eq!(
                        super::call_result_to_data_type(&value, $t.to_string()).unwrap(),
                        $e
                    );
                )*
            }
        }


        let test_data_32: [u8; 32] = [1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2];
        let test_data_64: [u8; 64] = [
            1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,
            1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2,3,4,5,6,7,8,9,0,1,2
        ];

        assert_data_types!(
            "u8", 1u8, "1".to_string(),
            "u16", 123u16, "123".to_string(),
            "u32", 9898u32, "9898".to_string(),
            "u64", 9999999999999u64, "9999999999999".to_string(),
            "u128", 111199999999999u128, "111199999999999".to_string(),
            "i8", -1i8, "-1".to_string(),
            "i16", -123i16, "-123".to_string(),
            "i32", -9898i32, "-9898".to_string(),
            "i64", -9999999999999i64, "-9999999999999".to_string(),
            "i128", -111199999999999i128, "-111199999999999".to_string(),
            "bool", false, "false".to_string(),
            "String", "asdas".to_string(), "\"asdas\"".to_string(),
            "Vec<u8>", [0u8, 1u8, 2u8].to_vec(), "[0, 1, 2]".to_string(),
            "Vec<u16>", [99u16].to_vec(), "[99]".to_string(),
            "Vec<u32>", [0u32, 6u32].to_vec(), "[0, 6]".to_string(),
            "Vec<u64>", [0u64, 6123123123u64].to_vec(), "[0, 6123123123]".to_string(),
            "Vec<i8>", [0i8, 1i8, -2i8].to_vec(), "[0, 1, -2]".to_string(),
            "Vec<i16>", [-99i16].to_vec(), "[-99]".to_string(),
            "Vec<i32>", [0i32, 6i32].to_vec(), "[0, 6]".to_string(),
            "Vec<i64>", [-1i64, -6123123123i64].to_vec(), "[-1, -6123123123]".to_string(),
            "Vec<i128>", [-1i128, -1i128, -1i128, -1i128, -6123123123i128].to_vec(), "[-1, -1, -1, -1, -6123123123]".to_string(),
            "Vec<bool>", [true, false, false].to_vec(), "[true, false, false]".to_string(),
            "Vec<String>", ["true", "false", "false"].to_vec(), "[\"true\", \"false\", \"false\"]".to_string(),
            "[ u 8; 3 2]", test_data_32, format!("{:?}", test_data_32),
            "[ u 8; 6 4]", test_data_64, format!("{:?}", test_data_64),
        );
    }
}
    
