# ParallelChain Mainnet Rust Client Library (pchain-client-rs).
`pchain-client-rs` is a library written in Rust that wraps around networked interactions with ParallelChain's HTTP APIs so that user does not have to manually produce request bodies and hit HTTP endpoints to serve information from ParallelChain.  

`pchain-client-rs` also provides some non-networked functionalities like storing information related to accounts (`wallet`), generating keypairs, hashing messages and parsing pchain_types::CallData.
 
## Initialize Pchain-Client  
Given a ParallelChain fullnode network provider URL, the following is the way we initialize an instance of `Client`:-

Usage: 
```rust
let client = Client::new("PARALLELCHAIN_FULLNODE_PROVIDER_URL");
```

The methods defined within `pchain-client-rs` are sub-divided into `networked` and `non-networked` methods. They are summarized below:-

## Networked methods
These methods require a network connection to be set up with ParallelChain network provider.
A few examples below illustrate the usage of the networked methods defined in `pchain-client-rs`.

### Account
#### Query Account State
Query account state for a ParalleChain public account.

Usage:
```rust
let sender_account: pchain_types::PublicAddress = pchain_types::Base64URL::decode("MUk3xs8F97RGmLvVfXfWMmq9X9FZWxUnbg7ToHc_Les")
    .expect("deserialization_failure").try_into().unwrap();

let state = pchain_client
                .state(&StateRequest {
                    accounts: HashSet::from([sender_address]),
                    include_contract: false,
                    storage_keys: HashMap::from([])
                })
                .await;

println!("{:?}", state);
```

Response:
```rust
Ok(StateResponse {
     accounts: {
         [49, 73, 55, 198, 207, 5, 247, 180, 70, 152, 187, 213, 125, 119, 214, 50, 106, 189, 95, 209, 89, 91, 21, 39, 110, 14, 211, 160, 119, 63, 45, 235]: 
         
         WithoutContract(AccountWithoutContract { 
             nonce: 10,
             balance: 1000, 
             cbi_version: None, 
             storage_hash: None 
        })
    },
    storage_tuples: {}, 
    block_hash: [96, 225, 116, 114, 128, 26, 7, 51, 229, 15, 85, 164, 174, 39, 115, 255, 49, 33, 13, 79, 192, 97, 250, 10, 94, 1, 95, 65, 174, 236, 230, 186]
})
```
### Block
#### Query Block by height

Usage:
```rust
let block_hash : [u8;32] = [239, 47, 233, 151, 38, 181, 179, 13, 204, 102, 141, 133, 96, 244, 115, 14, 42, 42, 105, 141, 253, 50, 126, 73, 48, 243, 26, 146, 222, 170, 126, 168];
let block: pchain_types::Block =  pchain_client
                            .block(&BlockRequest{ block_hash })
                            .await;;

println!("{:?}", block.header.height);
```

Response:
```rust
Ok(Some(
    Block {
        header: BlockHeader {
            hash: [239, 47, 233, 151, 38, 181, 179, 13, 204, 102, 141, 133, 96, 244, 115, 14, 42, 42, 105, 141, 253, 50, 126, 73, 48, 243, 26, 146, 222, 170, 126, 168]
            height: 100
            ...
        }
        transactions: ...
        receipts: ...
    }
))
```

### Transaction
#### Submit a Transaction of type `Transfer` to ParallelChain

Usage:
```rust
// retrieve the current active keypair from your wallet 
use pchain_client_rs::generate_keypair;

// prepare data for submission
let keypair = generate_keypair();
let nonce = 0;
let amount = 50;
let priority_fee_per_gas = 0;
let gas_limit = 67500000;
let max_base_fee_per_gas = 8;
let signer = pchain_types::Base64URL::decode("POikFlLT8sVuVt3RHJvxmzPKP8dfvi55TrME6Muc80I")?;
let receipient = pchain_types::Base64URL::decode("YrdzILDB_ocfZ0YElVfwbERtj9gcEE7Ue1RcIattToc")?;
let transaction_command = pchain_types::Command::Transfer { receipient,  amount: 50 };

let transaction = pchain_types::Transaction {
    commands,
    signer: origin,
    nonce: self.nonce,
    gas_limit: self.gas_limit,
    priority_fee_per_gas: self.priority_fee_per_gas,
    max_base_fee_per_gas: self.max_base_fee_per_gas, 
    hash: [0; 32],
    signature: [0; 64],
};

match transaction.to_signed(&keypair_raw) {
    Ok(signed_tx) => {
        // submit transaction to ParallelChain
        client.submit_transaction(
            signed_tx
        ) 
        .await?;
    },
    Err(e) => { 
        println!("Fail to sign transaction : {}", e));
    }
};

```

Response:
```rust
Ok(SubmitTransactionResponse {
    err: None
})
```
Or (as an example)
```rust
Ok(SubmitTransactionResponse {
    error: Some(MempoolFull)
})
```
Or (as an example)
```rust
Err("Internal Server Error. Server busy and failed to handle new request.")
```

For more examples on networked methods, please refer to the documentation on ParallelChain Mainnet.

#### Summary of all networked methods 
|Method|Description|
|:---|:---|
|**Type**|**Transaction**|
|`submit_transaction`|Submit a transaction to ParallelChain. Accepts `pchain_types::SignedTx` as an argument.|
|`view`|Call a method in a contract in a read-only way.|
|`transaction`| Get transaction by specifing transaction hash.|
|`receipt`|  Get receipt of transaction by specifing transaction hash.|
|`transaction_position`| Get transaction position in block by specifing transation hash.|
|**Type**|**Account**|
|`state`|Get the latest value associated with the specified key of a specific account. World state( balance, nonce, contract code) of the given account can also be returned. |
|**Type**|**Block**|
|`block`|Get block by specifing block hash|
|`block_header`|Get block header by specifing block hash.|
|`block_height_by_hash`|Get block by specifing block height.|
|`block_hash_by_height`|Get block hash by specifing block height.|
|`highest_committed_block`|Get the highest committed block.|
|**Type**|**Stake**|
|`validator_sets`|Return previous/current/next validator set. Validatot set operator, power, commission rate and stakes of operator/delegator are included.|
|`pools`|Return pool by specifying operators. Pool operator, power, commission rate and stakes of operator/delegator are included.|
|`stakes`|Return owner and power of stakes of the specified owner under specfied operators.|
|`deposits`|Return balance and auto stake rewards setting of the specified deposit account under specfied operators.|


## Non-networked methods
These methods do not require a ParallelChain network provider to operate.
A few examples below illustrate the usage of the non-networked methods defined in `pchain-client-rs`.

### Parser
A module for parsing base64 string, call arguments and call result.

The following example shows how to parse call arguments and call result.

Usage: 
```rust
use pchain_client_rs::{call_result_to_data_type, serialize_call_arrguments};

println!("Call Result to Data Type: {}", pchain_types::call_result_to_data_type("[11, 0, 0, 0]", "u32"));
println!("Serialized Call Arguments: {}", pchain_types::serialize_call_arrguments("[\"sss\", \"ddd\"]", "Vec< String >"));
```

Response:
```rust
"Call Result to Data Type:"  : 11
"Serialized Call Arguments:" : [2, 0, 0, 0, 3, 0, 0, 0, 115, 115, 115, 3, 0, 0, 0, 100, 100, 100]
```

#### Summary of methods provided by `Parser`
|Method|Description|
|:---|:---|
|`base64url_to_bytes32`| Decodes a Base64URL string into a slice of size 32.|
| `CallArguments::from_json`| Read from a string in json and deserialize it to vector of arg-type  to arg-value pairs. |
|`call_result_to_data_type`|  Deserialize call result to requested data type |
|`serialize_call_arrguments`| Serialize call arguments to bytes|


### Crypto
A module for cryptographic related operations.

The following example shows a utility to sign a message.

Usage: 
```rust
use pchain_client_rs::sign;

let keypair: String = "vxtLOMPDbUgh0czRj4Sg04Q2QGbJoh-klsN8yW-z5q-pfwc09zWo3V0S2HuRzLlCqzHD014DTceqRpJQ_3BIaQ";
println!("Ciphertext: {}", pchain_types::sign(keypair, "AAAA").to_string());

```

Response:
```rust
"Ciphertext: HIu-Y2cXALMaMcKyWDrpHdtrhW4zfDE2fmS9CHYZ8EplZ55WRqxQTL524cWvVLsVaWD5PPZeTQ1cGEH6lKrrAw"
```

#### Summary of methods provided by `Crypto`
|Method|Description|
|:---|:---|
|`sign`| Return cipherext cryptographically signed with supplied keypair.|
|`compute_contract_address`| Computes address of a contract from sender address and nonce | 
