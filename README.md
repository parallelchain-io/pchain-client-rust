# ParallelChain Client Library (Rust).

Rust client library for the [ParallelChain Protocol](https://github.com/parallelchain-io/parallelchain-protocol) fullnode RPC API.

## RPCs 

ParallelChain fullnodes expose three categories of RPCs:
- [Transaction-related RPCs](https://github.com/parallelchain-io/parallelchain-protocol): submit a transaction, query a transaction, its receipt, etc.
- [Block-related RPCs](https://github.com/parallelchain-io/parallelchain-protocol/blob/master/RPC.md#block-rpcs): query for blocks.
- [State-related RPCs](https://github.com/parallelchain-io/parallelchain-protocol/blob/master/RPC.md#state-rpcs): query the world state for contract code, the current validator set, deposits, etc.

## Versioning

The version of this library reflects the version of the ParallelChain Protocol which it implements. For example, the current version is 0.4.2, and this implements protocol version 0.4. Patch version increases are not guaranteed to be non-breaking.

## Opening an issue

Open an issue in GitHub if you:
1. Have a feature request / feature idea,
2. Have any questions (particularly software related questions),
3. Think you may have discovered a bug.

Please try to label your issues appropriately.