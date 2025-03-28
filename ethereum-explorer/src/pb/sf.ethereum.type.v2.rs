use serde::{Serialize, Deserialize};  // 引入序列化与反序列化宏
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct Block {
    /// Hash is the block's hash.
    #[prost(bytes="vec", tag="2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// Number is the block's height at which this block was mined.
    #[prost(uint64, tag="3")]
    pub number: u64,
    /// Size is the size in bytes of the RLP encoding of the block according to Ethereum
    /// rules.
    #[prost(uint64, tag="4")]
    pub size: u64,
    /// Header contain's the block's header information like its parent hash, the merkel root hash
    /// and all other information the form a block.
    #[prost(message, optional, tag="5")]
    pub header: ::core::option::Option<BlockHeader>,
    /// Uncles represents block produced with a valid solution but were not actually choosen
    /// as the canonical block for the given height so they are mostly "forked" blocks.
    ///
    /// If the Block has been produced using the Proof of Stake consensus algorithm, this
    /// field will actually be always empty.
    #[prost(message, repeated, tag="6")]
    pub uncles: ::prost::alloc::vec::Vec<BlockHeader>,
    /// TransactionTraces hold the execute trace of all the transactions that were executed
    /// in this block. In in there that you will find most of the Ethereum data model.
    ///
    /// They are ordered by the order of execution of the transaction in the block.
    #[prost(message, repeated, tag="10")]
    pub transaction_traces: ::prost::alloc::vec::Vec<TransactionTrace>,
    /// BalanceChanges here is the array of ETH transfer that happened at the block level
    /// outside of the normal transaction flow of a block. The best example of this is mining
    /// reward for the block mined, the transfer of ETH to the miner happens outside the normal
    /// transaction flow of the chain and is recorded as a `BalanceChange` here since we cannot
    /// attached it to any transaction.
    ///
    /// Only available in DetailLevel: EXTENDED
    #[prost(message, repeated, tag="11")]
    pub balance_changes: ::prost::alloc::vec::Vec<BalanceChange>,
    /// DetailLevel affects the data available in this block.
    ///
    /// ## DetailLevel_EXTENDED
    ///
    /// Describes the most complete block, with traces, balance changes, storage
    /// changes. It is extracted during the execution of the block.
    ///
    /// ## DetailLevel_BASE
    ///
    /// Describes a block that contains only the block header, transaction receipts
    /// and event logs: everything that can be extracted using the base JSON-RPC
    /// interface
    /// (<https://ethereum.org/en/developers/docs/apis/json-rpc/#json-rpc-methods>)
    /// Furthermore, the eth_getTransactionReceipt call has been avoided because it
    /// brings only minimal improvements at the cost of requiring an archive node
    /// or a full node with complete transaction index.
    #[prost(enumeration="block::DetailLevel", tag="12")]
    pub detail_level: i32,
    /// CodeChanges here is the array of smart code change that happened that happened at the block level
    /// outside of the normal transaction flow of a block. Some Ethereum's fork like BSC and Polygon
    /// has some capabilities to upgrade internal smart contracts used usually to track the validator
    /// list.
    ///
    /// On hard fork, some procedure runs to upgrade the smart contract code to a new version. In those
    /// network, a `CodeChange` for each modified smart contract on upgrade would be present here. Note
    /// that this happen rarely, so the vast majority of block will have an empty list here.
    ///
    /// Only available in DetailLevel: EXTENDED
    #[prost(message, repeated, tag="20")]
    pub code_changes: ::prost::alloc::vec::Vec<CodeChange>,
    /// System calls are introduced in Cancun, along with blobs. They are executed outside of transactions but affect the state.
    ///
    /// Only available in DetailLevel: EXTENDED
    #[prost(message, repeated, tag="21")]
    pub system_calls: ::prost::alloc::vec::Vec<Call>,
    /// Ver represents that data model version of the block, it is used internally by Firehose on Ethereum
    /// as a validation that we are reading the correct version.
    #[prost(int32, tag="1")]
    pub ver: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct Timestamp {
    /// Represents seconds of UTC time since Unix epoch
    /// 1970-01-01T00:00:00Z. Must be from 0001-01-01T00:00:00Z to
    /// 9999-12-31T23:59:59Z inclusive.
    #[prost(int64, tag = "1")]
    pub seconds: i64,
    /// Non-negative fractions of a second at nanosecond resolution. Negative
    /// second values with fractions must still have non-negative nanos values
    /// that count forward in time. Must be from 0 to 999,999,999
    /// inclusive.
    #[prost(int32, tag = "2")]
    pub nanos: i32,
}

/// Nested message and enum types in `Block`.
pub mod block {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DetailLevel {
        DetaillevelExtended = 0,
        /// DETAILLEVEL_TRACE = 1; // TBD
        DetaillevelBase = 2,
    }
    impl DetailLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DetailLevel::DetaillevelExtended => "DETAILLEVEL_EXTENDED",
                DetailLevel::DetaillevelBase => "DETAILLEVEL_BASE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DETAILLEVEL_EXTENDED" => Some(Self::DetaillevelExtended),
                "DETAILLEVEL_BASE" => Some(Self::DetaillevelBase),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct BlockHeader {
    #[prost(bytes="vec", tag="1")]
    pub parent_hash: ::prost::alloc::vec::Vec<u8>,
    /// Uncle hash of the block, some reference it as `sha3Uncles`, but `sha3`` is badly worded, so we prefer `uncle_hash`, also
    /// referred as `ommers` in EIP specification.
    ///
    /// If the Block containing this `BlockHeader` has been produced using the Proof of Stake
    /// consensus algorithm, this field will actually be constant and set to `0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347`.
    #[prost(bytes="vec", tag="2")]
    pub uncle_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub coinbase: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub state_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub transactions_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub receipt_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub logs_bloom: ::prost::alloc::vec::Vec<u8>,
    /// Difficulty is the difficulty of the Proof of Work algorithm that was required to compute a solution.
    ///
    /// If the Block containing this `BlockHeader` has been produced using the Proof of Stake
    /// consensus algorithm, this field will actually be constant and set to `0x00`.
    #[prost(message, optional, tag="8")]
    pub difficulty: ::core::option::Option<BigInt>,
    /// TotalDifficulty is the sum of all previous blocks difficulty including this block difficulty.
    ///
    /// If the Block containing this `BlockHeader` has been produced using the Proof of Stake
    /// consensus algorithm, this field will actually be constant and set to the terminal total difficulty
    /// that was required to transition to Proof of Stake algorithm, which varies per network. It is set to
    /// 58 750 000 000 000 000 000 000 on Ethereum Mainnet and to 10 790 000 on Ethereum Testnet Goerli.
    #[prost(message, optional, tag="17")]
    pub total_difficulty: ::core::option::Option<BigInt>,
    #[prost(uint64, tag="9")]
    pub number: u64,
    #[prost(uint64, tag="10")]
    pub gas_limit: u64,
    #[prost(uint64, tag="11")]
    pub gas_used: u64,
    #[prost(message, optional, tag="12")]
    pub timestamp: ::core::option::Option<Timestamp>,
    /// ExtraData is free-form bytes included in the block by the "miner". While on Yellow paper of
    /// Ethereum this value is maxed to 32 bytes, other consensus algorithm like Clique and some other
    /// forks are using bigger values to carry special consensus data.
    ///
    /// If the Block containing this `BlockHeader` has been produced using the Proof of Stake
    /// consensus algorithm, this field is strictly enforced to be <= 32 bytes.
    #[prost(bytes="vec", tag="13")]
    pub extra_data: ::prost::alloc::vec::Vec<u8>,
    /// MixHash is used to prove, when combined with the `nonce` that sufficient amount of computation has been
    /// achieved and that the solution found is valid.
    #[prost(bytes="vec", tag="14")]
    pub mix_hash: ::prost::alloc::vec::Vec<u8>,
    /// Nonce is used to prove, when combined with the `mix_hash` that sufficient amount of computation has been
    /// achieved and that the solution found is valid.
    ///
    /// If the Block containing this `BlockHeader` has been produced using the Proof of Stake
    /// consensus algorithm, this field will actually be constant and set to `0`.
    #[prost(uint64, tag="15")]
    pub nonce: u64,
    /// Hash is the hash of the block which is actually the computation:
    ///
    ///   Keccak256(rlp([
    ///     parent_hash,
    ///     uncle_hash,
    ///     coinbase,
    ///     state_root,
    ///     transactions_root,
    ///     receipt_root,
    ///     logs_bloom,
    ///     difficulty,
    ///     number,
    ///     gas_limit,
    ///     gas_used,
    ///     timestamp,
    ///     extra_data,
    ///     mix_hash,
    ///     nonce,
    ///     base_fee_per_gas (to be included only if London fork is active)
    ///     withdrawals_root (to be included only if Shangai fork is active)
    ///     blob_gas_used (to be included only if Cancun fork is active)
    ///     excess_blob_gas (to be included only if Cancun fork is active)
    ///     parent_beacon_root (to be included only if Cancun fork is active)
    ///   ]))
    ///
    #[prost(bytes="vec", tag="16")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// Base fee per gas according to EIP-1559 (e.g. London Fork) rules, only set if London is present/active on the chain.
    #[prost(message, optional, tag="18")]
    pub base_fee_per_gas: ::core::option::Option<BigInt>,
    /// Withdrawals root hash according to EIP-4895 (e.g. Shangai Fork) rules, only set if Shangai is present/active on the chain.
    ///
    /// Only available in DetailLevel: EXTENDED
    #[prost(bytes="vec", tag="19")]
    pub withdrawals_root: ::prost::alloc::vec::Vec<u8>,
    /// Only available in DetailLevel: EXTENDED
    #[prost(message, optional, tag="20")]
    pub tx_dependency: ::core::option::Option<Uint64NestedArray>,
    /// BlobGasUsed was added by EIP-4844 and is ignored in legacy headers.
    #[prost(uint64, optional, tag="22")]
    pub blob_gas_used: ::core::option::Option<u64>,
    /// ExcessBlobGas was added by EIP-4844 and is ignored in legacy headers.
    #[prost(uint64, optional, tag="23")]
    pub excess_blob_gas: ::core::option::Option<u64>,
    /// ParentBeaconRoot was added by EIP-4788 and is ignored in legacy headers.
    #[prost(bytes="vec", tag="24")]
    pub parent_beacon_root: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct Uint64NestedArray {
    #[prost(message, repeated, tag="1")]
    pub val: ::prost::alloc::vec::Vec<Uint64Array>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct Uint64Array {
    #[prost(uint64, repeated, tag="1")]
    pub val: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct BigInt {
    #[prost(bytes="vec", tag="1")]
    pub bytes: ::prost::alloc::vec::Vec<u8>,
}
/// TransactionTrace is full trace of execution of the transaction when the
/// it actually executed on chain.
///
/// It contains all the transaction details like `from`, `to`, `gas`, etc.
/// as well as all the internal calls that were made during the transaction.
///
/// The `calls` vector contains Call objects which have balance changes, events
/// storage changes, etc.
///
/// If ordering is important between elements, almost each message like `Log`,
/// `Call`, `StorageChange`, etc. have an ordinal field that is represents "execution"
/// order of the said element against all other elements in this block.
///
/// Due to how the call tree works doing "naively", looping through all calls then
/// through a Call's element like `logs` while not yielding the elements in the order
/// they were executed on chain. A log in call could have been done before or after
/// another in another call depending on the actual call tree.
///
/// The `calls` are ordered by creation order and the call tree can be re-computing
/// using fields found in `Call` object (parent/child relationship).
///
/// Another important thing to note is that even if a transaction succeed, some calls
/// within it could have been reverted internally, if this is important to you, you must
/// check the field `state_reverted` on the `Call` to determine if it was fully committed
/// to the chain or not.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct TransactionTrace {
    /// consensus
    #[prost(bytes="vec", tag="1")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub nonce: u64,
    /// GasPrice represents the effective price that has been paid for each gas unit of this transaction. Over time, the
    /// Ethereum rules changes regarding GasPrice field here. Before London fork, the GasPrice was always set to the
    /// fixed gas price. After London fork, this value has different meaning depending on the transaction type (see `Type` field).
    ///
    /// In cases where `TransactionTrace.Type == TRX_TYPE_LEGACY || TRX_TYPE_ACCESS_LIST`, then GasPrice has the same meaning
    /// as before the London fork.
    ///
    /// In cases where `TransactionTrace.Type == TRX_TYPE_DYNAMIC_FEE`, then GasPrice is the effective gas price paid
    /// for the transaction which is equals to `BlockHeader.BaseFeePerGas + TransactionTrace.`
    #[prost(message, optional, tag="3")]
    pub gas_price: ::core::option::Option<BigInt>,
    /// GasLimit is the maximum of gas unit the sender of the transaction is willing to consume when perform the EVM
    /// execution of the whole transaction
    #[prost(uint64, tag="4")]
    pub gas_limit: u64,
    /// Value is the amount of Ether transferred as part of this transaction.
    #[prost(message, optional, tag="5")]
    pub value: ::core::option::Option<BigInt>,
    /// Input data the transaction will receive for execution of EVM.
    #[prost(bytes="vec", tag="6")]
    pub input: ::prost::alloc::vec::Vec<u8>,
    /// V is the recovery ID value for the signature Y point.
    #[prost(bytes="vec", tag="7")]
    pub v: ::prost::alloc::vec::Vec<u8>,
    /// R is the signature's X point on the elliptic curve (32 bytes).
    #[prost(bytes="vec", tag="8")]
    pub r: ::prost::alloc::vec::Vec<u8>,
    /// S is the signature's Y point on the elliptic curve (32 bytes).
    #[prost(bytes="vec", tag="9")]
    pub s: ::prost::alloc::vec::Vec<u8>,
    /// GasUsed is the total amount of gas unit used for the whole execution of the transaction.
    #[prost(uint64, tag="10")]
    pub gas_used: u64,
    /// Type represents the Ethereum transaction type, available only since EIP-2718 & EIP-2930 activation which happened on Berlin fork.
    /// The value is always set even for transaction before Berlin fork because those before the fork are still legacy transactions.
    #[prost(enumeration="transaction_trace::Type", tag="12")]
    pub r#type: i32,
    /// AcccessList represents the storage access this transaction has agreed to do in which case those storage
    /// access cost less gas unit per access.
    ///
    /// This will is populated only if `TransactionTrace.Type == TRX_TYPE_ACCESS_LIST || TRX_TYPE_DYNAMIC_FEE` which
    /// is possible only if Berlin (TRX_TYPE_ACCESS_LIST) nor London (TRX_TYPE_DYNAMIC_FEE) fork are active on the chain.
    #[prost(message, repeated, tag="14")]
    pub access_list: ::prost::alloc::vec::Vec<AccessTuple>,
    /// MaxFeePerGas is the maximum fee per gas the user is willing to pay for the transaction gas used.
    ///
    /// This will is populated only if `TransactionTrace.Type == TRX_TYPE_DYNAMIC_FEE` which is possible only
    /// if Londong fork is active on the chain.
    ///
    /// Only available in DetailLevel: EXTENDED
    #[prost(message, optional, tag="11")]
    pub max_fee_per_gas: ::core::option::Option<BigInt>,
    /// MaxPriorityFeePerGas is priority fee per gas the user to pay in extra to the miner on top of the block's
    /// base fee.
    ///
    /// This will is populated only if `TransactionTrace.Type == TRX_TYPE_DYNAMIC_FEE` which is possible only
    /// if London fork is active on the chain.
    ///
    /// Only available in DetailLevel: EXTENDED
    #[prost(message, optional, tag="13")]
    pub max_priority_fee_per_gas: ::core::option::Option<BigInt>,
    /// meta
    #[prost(uint32, tag="20")]
    pub index: u32,
    #[prost(bytes="vec", tag="21")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="22")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    /// Only available in DetailLevel: EXTENDED
    #[prost(bytes="vec", tag="23")]
    pub return_data: ::prost::alloc::vec::Vec<u8>,
    /// Only available in DetailLevel: EXTENDED
    #[prost(bytes="vec", tag="24")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
    /// The block's global ordinal when the transaction started executing, refer to
    /// \[Block\] documentation for further information about ordinals and total ordering.
    #[prost(uint64, tag="25")]
    pub begin_ordinal: u64,
    /// The block's global ordinal when the transaction finished executing, refer to
    /// \[Block\] documentation for further information about ordinals and total ordering.
    #[prost(uint64, tag="26")]
    pub end_ordinal: u64,
    /// TransactionTraceStatus is the status of the transaction execution and will let you know if the transaction
    /// was successful or not.
    ///
    /// A successful transaction has been recorded to the blockchain's state for calls in it that were successful.
    /// This means it's possible only a subset of the calls were properly recorded, refer to \[calls[].state_reverted\] field
    /// to determine which calls were reverted.
    ///
    /// A quirks of the Ethereum protocol is that a transaction `FAILED` or `REVERTED` still affects the blockchain's
    /// state for **some** of the state changes. Indeed, in those cases, the transactions fees are still paid to the miner
    /// which means there is a balance change for the transaction's emitter (e.g. `from`) to pay the gas fees, an optional
    /// balance change for gas refunded to the transaction's emitter (e.g. `from`) and a balance change for the miner who
    /// received the transaction fees. There is also a nonce change for the transaction's emitter (e.g. `from`).
    ///
    /// This means that to properly record the state changes for a transaction, you need to conditionally procees the
    /// transaction's status.
    ///
    /// For a `SUCCEEDED` transaction, you iterate over the `calls` array and record the state changes for each call for
    /// which `state_reverted == false` (if a transaction succeeded, the call at #0 will always `state_reverted == false`
    /// because it aligns with the transaction).
    ///
    /// For a `FAILED` or `REVERTED` transaction, you iterate over the root call (e.g. at #0, will always exist) for
    /// balance changes you process those where `reason` is either `REASON_GAS_BUY`, `REASON_GAS_REFUND` or
    /// `REASON_REWARD_TRANSACTION_FEE` and for nonce change, still on the root call, you pick the nonce change which the
    /// smallest ordinal (if more than one).
    #[prost(enumeration="TransactionTraceStatus", tag="30")]
    pub status: i32,
    #[prost(message, optional, tag="31")]
    pub receipt: ::core::option::Option<TransactionReceipt>,
    /// Only available in DetailLevel: EXTENDED
    #[prost(message, repeated, tag="32")]
    pub calls: ::prost::alloc::vec::Vec<Call>,
    /// BlobGas is the amount of gas the transaction is going to pay for the blobs, this is a computed value
    /// equivalent to `self.blob_gas_fee_cap * len(self.blob_hashes)` and provided in the model for convenience.
    ///
    /// This is specified by <https://eips.ethereum.org/EIPS/eip-4844>
    ///
    /// This will is populated only if `TransactionTrace.Type == TRX_TYPE_BLOB` which is possible only
    /// if Cancun fork is active on the chain.
    #[prost(uint64, optional, tag="33")]
    pub blob_gas: ::core::option::Option<u64>,
    /// BlobGasFeeCap is the maximum fee per data gas the user is willing to pay for the data gas used.
    ///
    /// This is specified by <https://eips.ethereum.org/EIPS/eip-4844>
    ///
    /// This will is populated only if `TransactionTrace.Type == TRX_TYPE_BLOB` which is possible only
    /// if Cancun fork is active on the chain.
    #[prost(message, optional, tag="34")]
    pub blob_gas_fee_cap: ::core::option::Option<BigInt>,
    /// BlobHashes field represents a list of hash outputs from 'kzg_to_versioned_hash' which
    /// essentially is a version byte + the sha256 hash of the blob commitment (e.g.
    /// `BLOB_COMMITMENT_VERSION_KZG + sha256(commitment)\[1:\]`.
    ///
    /// This is specified by <https://eips.ethereum.org/EIPS/eip-4844>
    ///
    /// This will is populated only if `TransactionTrace.Type == TRX_TYPE_BLOB` which is possible only
    /// if Cancun fork is active on the chain.
    #[prost(bytes="vec", repeated, tag="35")]
    pub blob_hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `TransactionTrace`.
pub mod transaction_trace {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,serde::Serialize, serde::Deserialize)]
    #[repr(i32)]
    pub enum Type {
        /// All transactions that ever existed prior Berlin fork before EIP-2718 was implemented.
        TrxTypeLegacy = 0,
        /// Transaction that specicy an access list of contract/storage_keys that is going to be used
        /// in this transaction.
        ///
        /// Added in Berlin fork (EIP-2930).
        TrxTypeAccessList = 1,
        /// Transaction that specifis an access list just like TRX_TYPE_ACCESS_LIST but in addition defines the
        /// max base gas gee and max priority gas fee to pay for this transaction. Transaction's of those type are
        /// executed against EIP-1559 rules which dictates a dynamic gas cost based on the congestion of the network.
        TrxTypeDynamicFee = 2,
        /// Transaction which contain a large amount of data that cannot be accessed by EVM execution, but whose commitment
        /// can be accessed. The format is intended to be fully compatible with the format that will be used in full sharding.
        ///
        /// Transaction that defines specifis an access list just like TRX_TYPE_ACCESS_LIST and enables dynamic fee just like
        /// TRX_TYPE_DYNAMIC_FEE but in addition defines the fields 'max_fee_per_data_gas' of type 'uint256' and the fields
        /// 'blob_versioned_hashes' field represents a list of hash outputs from 'kzg_to_versioned_hash'.
        ///
        /// Activated in Dencun
        TrxTypeBlob = 3,
        /// Arbitrum-specific transactions
        TrxTypeArbitrumDeposit = 100,
        TrxTypeArbitrumUnsigned = 101,
        TrxTypeArbitrumContract = 102,
        TrxTypeArbitrumRetry = 104,
        TrxTypeArbitrumSubmitRetryable = 105,
        TrxTypeArbitrumInternal = 106,
        TrxTypeArbitrumLegacy = 120,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::TrxTypeLegacy => "TRX_TYPE_LEGACY",
                Type::TrxTypeAccessList => "TRX_TYPE_ACCESS_LIST",
                Type::TrxTypeDynamicFee => "TRX_TYPE_DYNAMIC_FEE",
                Type::TrxTypeBlob => "TRX_TYPE_BLOB",
                Type::TrxTypeArbitrumDeposit => "TRX_TYPE_ARBITRUM_DEPOSIT",
                Type::TrxTypeArbitrumUnsigned => "TRX_TYPE_ARBITRUM_UNSIGNED",
                Type::TrxTypeArbitrumContract => "TRX_TYPE_ARBITRUM_CONTRACT",
                Type::TrxTypeArbitrumRetry => "TRX_TYPE_ARBITRUM_RETRY",
                Type::TrxTypeArbitrumSubmitRetryable => "TRX_TYPE_ARBITRUM_SUBMIT_RETRYABLE",
                Type::TrxTypeArbitrumInternal => "TRX_TYPE_ARBITRUM_INTERNAL",
                Type::TrxTypeArbitrumLegacy => "TRX_TYPE_ARBITRUM_LEGACY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TRX_TYPE_LEGACY" => Some(Self::TrxTypeLegacy),
                "TRX_TYPE_ACCESS_LIST" => Some(Self::TrxTypeAccessList),
                "TRX_TYPE_DYNAMIC_FEE" => Some(Self::TrxTypeDynamicFee),
                "TRX_TYPE_BLOB" => Some(Self::TrxTypeBlob),
                "TRX_TYPE_ARBITRUM_DEPOSIT" => Some(Self::TrxTypeArbitrumDeposit),
                "TRX_TYPE_ARBITRUM_UNSIGNED" => Some(Self::TrxTypeArbitrumUnsigned),
                "TRX_TYPE_ARBITRUM_CONTRACT" => Some(Self::TrxTypeArbitrumContract),
                "TRX_TYPE_ARBITRUM_RETRY" => Some(Self::TrxTypeArbitrumRetry),
                "TRX_TYPE_ARBITRUM_SUBMIT_RETRYABLE" => Some(Self::TrxTypeArbitrumSubmitRetryable),
                "TRX_TYPE_ARBITRUM_INTERNAL" => Some(Self::TrxTypeArbitrumInternal),
                "TRX_TYPE_ARBITRUM_LEGACY" => Some(Self::TrxTypeArbitrumLegacy),
                _ => None,
            }
        }
    }
}
/// AccessTuple represents a list of storage keys for a given contract's address and is used
/// for AccessList construction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct AccessTuple {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub storage_keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct TransactionReceipt {
    /// State root is an intermediate state_root hash, computed in-between transactions to make
    /// **sure** you could build a proof and point to state in the middle of a block. Geth client
    /// uses `PostState + root + PostStateOrStatus`` while Parity used `status_code, root...`` this piles
    /// hardforks, see (read the EIPs first):
    /// - <https://github.com/ethereum/EIPs/blob/master/EIPS/eip-658.md>
    ///
    /// Moreover, the notion of `Outcome`` in parity, which segregates the two concepts, which are
    /// stored in the same field `status_code`` can be computed based on such a hack of the `state_root`
    /// field, following `EIP-658`.
    ///
    /// Before Byzantinium hard fork, this field is always empty.
    #[prost(bytes="vec", tag="1")]
    pub state_root: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub cumulative_gas_used: u64,
    #[prost(bytes="vec", tag="3")]
    pub logs_bloom: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="4")]
    pub logs: ::prost::alloc::vec::Vec<Log>,
    /// BlobGasUsed is the amount of blob gas that has been used within this transaction. At time
    /// of writing, this is equal to `self.blob_gas_fee_cap * len(self.blob_hashes)`.
    ///
    /// This is specified by <https://eips.ethereum.org/EIPS/eip-4844>
    ///
    /// This will is populated only if `TransactionTrace.Type == TRX_TYPE_BLOB` which is possible only
    /// if Cancun fork is active on the chain.
    #[prost(uint64, optional, tag="5")]
    pub blob_gas_used: ::core::option::Option<u64>,
    /// BlobGasPrice is the amount to pay per blob item in the transaction.
    ///
    /// This is specified by <https://eips.ethereum.org/EIPS/eip-4844>
    ///
    /// This will is populated only if `TransactionTrace.Type == TRX_TYPE_BLOB` which is possible only
    /// if Cancun fork is active on the chain.
    #[prost(message, optional, tag="6")]
    pub blob_gas_price: ::core::option::Option<BigInt>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct Log {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub topics: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Index is the index of the log relative to the transaction. This index
    /// is always populated regardless of the state revertion of the the call
    /// that emitted this log.
    ///
    /// Only available in DetailLevel: EXTENDED
    #[prost(uint32, tag="4")]
    pub index: u32,
    /// BlockIndex represents the index of the log relative to the Block.
    ///
    /// An **important** notice is that this field will be 0 when the call
    /// that emitted the log has been reverted by the chain.
    ///
    /// Currently, there is two locations where a Log can be obtained:
    /// - block.transaction_traces\[].receipt.logs[\]
    /// - block.transaction_traces\[].calls[].logs[\]
    ///
    /// In the `receipt` case, the logs will be populated only when the call
    /// that emitted them has not been reverted by the chain and when in this
    /// position, the `blockIndex` is always populated correctly.
    ///
    /// In the case of `calls` case, for `call` where `stateReverted == true`,
    /// the `blockIndex` value will always be 0.
    #[prost(uint32, tag="6")]
    pub block_index: u32,
    /// The block's global ordinal when the log was recorded, refer to \[Block\]
    /// documentation for further information about ordinals and total ordering.
    #[prost(uint64, tag="7")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct Call {
    #[prost(uint32, tag="1")]
    pub index: u32,
    #[prost(uint32, tag="2")]
    pub parent_index: u32,
    #[prost(uint32, tag="3")]
    pub depth: u32,
    #[prost(enumeration="CallType", tag="4")]
    pub call_type: i32,
    #[prost(bytes="vec", tag="5")]
    pub caller: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="7")]
    pub value: ::core::option::Option<BigInt>,
    #[prost(uint64, tag="8")]
    pub gas_limit: u64,
    #[prost(uint64, tag="9")]
    pub gas_consumed: u64,
    #[prost(bytes="vec", tag="13")]
    pub return_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="14")]
    pub input: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="15")]
    pub executed_code: bool,
    #[prost(bool, tag="16")]
    pub suicide: bool,
    /// hex representation of the hash -> preimage 
    #[prost(map="string, string", tag="20")]
    pub keccak_preimages: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(message, repeated, tag="21")]
    pub storage_changes: ::prost::alloc::vec::Vec<StorageChange>,
    #[prost(message, repeated, tag="22")]
    pub balance_changes: ::prost::alloc::vec::Vec<BalanceChange>,
    #[prost(message, repeated, tag="24")]
    pub nonce_changes: ::prost::alloc::vec::Vec<NonceChange>,
    #[prost(message, repeated, tag="25")]
    pub logs: ::prost::alloc::vec::Vec<Log>,
    #[prost(message, repeated, tag="26")]
    pub code_changes: ::prost::alloc::vec::Vec<CodeChange>,
    #[prost(message, repeated, tag="28")]
    pub gas_changes: ::prost::alloc::vec::Vec<GasChange>,
    /// In Ethereum, a call can be either:
    /// - Successfull, execution passes without any problem encountered
    /// - Failed, execution failed, and remaining gas should be consumed
    /// - Reverted, execution failed, but only gas consumed so far is billed, remaining gas is refunded
    ///
    /// When a call is either `failed` or `reverted`, the `status_failed` field
    /// below is set to `true`. If the status is `reverted`, then both `status_failed`
    /// and `status_reverted` are going to be set to `true`.
    #[prost(bool, tag="10")]
    pub status_failed: bool,
    #[prost(bool, tag="12")]
    pub status_reverted: bool,
    /// Populated when a call either failed or reverted, so when `status_failed == true`,
    /// see above for details about those flags.
    #[prost(string, tag="11")]
    pub failure_reason: ::prost::alloc::string::String,
    /// This field represents wheter or not the state changes performed
    /// by this call were correctly recorded by the blockchain.
    ///
    /// On Ethereum, a transaction can record state changes even if some
    /// of its inner nested calls failed. This is problematic however since
    /// a call will invalidate all its state changes as well as all state
    /// changes performed by its child call. This means that even if a call
    /// has a status of `SUCCESS`, the chain might have reverted all the state
    /// changes it performed.
    ///
    /// ```text
    ///    Trx 1
    ///     Call #1 <Failed>
    ///       Call #2 <Execution Success>
    ///       Call #3 <Execution Success>
    ///       |--- Failure here
    ///     Call #4
    /// ```
    ///
    /// In the transaction above, while Call #2 and Call #3 would have the
    /// status `EXECUTED`.
    ///
    /// If you check all calls and check only `state_reverted` flag, you might be missing
    /// some balance changes and nonce changes. This is because when a full transaction fails
    /// in ethereum (e.g. `calls.all(x.state_reverted == true)`), there is still the transaction
    /// fee that are recorded to the chain.
    ///
    /// Refer to \[TransactionTrace#status\] field for more details about the handling you must
    /// perform.
    #[prost(bool, tag="30")]
    pub state_reverted: bool,
    /// The block's global ordinal when the call started executing, refer to
    /// \[Block\] documentation for further information about ordinals and total ordering.
    #[prost(uint64, tag="31")]
    pub begin_ordinal: u64,
    /// The block's global ordinal when the call finished executing, refer to
    /// \[Block\] documentation for further information about ordinals and total ordering.
    #[prost(uint64, tag="32")]
    pub end_ordinal: u64,
    #[prost(message, repeated, tag="33")]
    pub account_creations: ::prost::alloc::vec::Vec<AccountCreation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct StorageChange {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub old_value: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub new_value: ::prost::alloc::vec::Vec<u8>,
    /// The block's global ordinal when the storage change was recorded, refer to \[Block\]
    /// documentation for further information about ordinals and total ordering.
    #[prost(uint64, tag="5")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct BalanceChange {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub old_value: ::core::option::Option<BigInt>,
    #[prost(message, optional, tag="3")]
    pub new_value: ::core::option::Option<BigInt>,
    #[prost(enumeration="balance_change::Reason", tag="4")]
    pub reason: i32,
    /// The block's global ordinal when the balance change was recorded, refer to \[Block\]
    /// documentation for further information about ordinals and total ordering.
    #[prost(uint64, tag="5")]
    pub ordinal: u64,
}
/// Nested message and enum types in `BalanceChange`.
pub mod balance_change {
    /// Obtain all balanche change reasons under deep mind repository:
    ///
    /// ```shell
    /// ack -ho 'BalanceChangeReason\(".*"\)' | grep -Eo '".*"' | sort | uniq
    /// ```
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,serde::Serialize, serde::Deserialize)]
    #[repr(i32)]
    pub enum Reason {
        Unknown = 0,
        RewardMineUncle = 1,
        RewardMineBlock = 2,
        DaoRefundContract = 3,
        DaoAdjustBalance = 4,
        Transfer = 5,
        GenesisBalance = 6,
        GasBuy = 7,
        RewardTransactionFee = 8,
        RewardFeeReset = 14,
        GasRefund = 9,
        TouchAccount = 10,
        SuicideRefund = 11,
        SuicideWithdraw = 13,
        CallBalanceOverride = 12,
        /// Used on chain(s) where some Ether burning happens
        Burn = 15,
        Withdrawal = 16,
    }
    impl Reason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Reason::Unknown => "REASON_UNKNOWN",
                Reason::RewardMineUncle => "REASON_REWARD_MINE_UNCLE",
                Reason::RewardMineBlock => "REASON_REWARD_MINE_BLOCK",
                Reason::DaoRefundContract => "REASON_DAO_REFUND_CONTRACT",
                Reason::DaoAdjustBalance => "REASON_DAO_ADJUST_BALANCE",
                Reason::Transfer => "REASON_TRANSFER",
                Reason::GenesisBalance => "REASON_GENESIS_BALANCE",
                Reason::GasBuy => "REASON_GAS_BUY",
                Reason::RewardTransactionFee => "REASON_REWARD_TRANSACTION_FEE",
                Reason::RewardFeeReset => "REASON_REWARD_FEE_RESET",
                Reason::GasRefund => "REASON_GAS_REFUND",
                Reason::TouchAccount => "REASON_TOUCH_ACCOUNT",
                Reason::SuicideRefund => "REASON_SUICIDE_REFUND",
                Reason::SuicideWithdraw => "REASON_SUICIDE_WITHDRAW",
                Reason::CallBalanceOverride => "REASON_CALL_BALANCE_OVERRIDE",
                Reason::Burn => "REASON_BURN",
                Reason::Withdrawal => "REASON_WITHDRAWAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REASON_UNKNOWN" => Some(Self::Unknown),
                "REASON_REWARD_MINE_UNCLE" => Some(Self::RewardMineUncle),
                "REASON_REWARD_MINE_BLOCK" => Some(Self::RewardMineBlock),
                "REASON_DAO_REFUND_CONTRACT" => Some(Self::DaoRefundContract),
                "REASON_DAO_ADJUST_BALANCE" => Some(Self::DaoAdjustBalance),
                "REASON_TRANSFER" => Some(Self::Transfer),
                "REASON_GENESIS_BALANCE" => Some(Self::GenesisBalance),
                "REASON_GAS_BUY" => Some(Self::GasBuy),
                "REASON_REWARD_TRANSACTION_FEE" => Some(Self::RewardTransactionFee),
                "REASON_REWARD_FEE_RESET" => Some(Self::RewardFeeReset),
                "REASON_GAS_REFUND" => Some(Self::GasRefund),
                "REASON_TOUCH_ACCOUNT" => Some(Self::TouchAccount),
                "REASON_SUICIDE_REFUND" => Some(Self::SuicideRefund),
                "REASON_SUICIDE_WITHDRAW" => Some(Self::SuicideWithdraw),
                "REASON_CALL_BALANCE_OVERRIDE" => Some(Self::CallBalanceOverride),
                "REASON_BURN" => Some(Self::Burn),
                "REASON_WITHDRAWAL" => Some(Self::Withdrawal),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct NonceChange {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub old_value: u64,
    #[prost(uint64, tag="3")]
    pub new_value: u64,
    /// The block's global ordinal when the nonce change was recorded, refer to \[Block\]
    /// documentation for further information about ordinals and total ordering.
    #[prost(uint64, tag="4")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct AccountCreation {
    #[prost(bytes="vec", tag="1")]
    pub account: ::prost::alloc::vec::Vec<u8>,
    /// The block's global ordinal when the account creation was recorded, refer to \[Block\]
    /// documentation for further information about ordinals and total ordering.
    #[prost(uint64, tag="2")]
    pub ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct CodeChange {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub old_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub old_code: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub new_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub new_code: ::prost::alloc::vec::Vec<u8>,
    /// The block's global ordinal when the code change was recorded, refer to \[Block\]
    /// documentation for further information about ordinals and total ordering.
    #[prost(uint64, tag="6")]
    pub ordinal: u64,
}
/// The gas change model represents the reason why some gas cost has occurred.
/// The gas is computed per actual op codes. Doing them completely might prove
/// overwhelming in most cases.
///
/// Hence, we only index some of them, those that are costy like all the calls
/// one, log events, return data, etc.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct GasChange {
    #[prost(uint64, tag="1")]
    pub old_value: u64,
    #[prost(uint64, tag="2")]
    pub new_value: u64,
    #[prost(enumeration="gas_change::Reason", tag="3")]
    pub reason: i32,
    /// The block's global ordinal when the gas change was recorded, refer to \[Block\]
    /// documentation for further information about ordinals and total ordering.
    #[prost(uint64, tag="4")]
    pub ordinal: u64,
}
/// Nested message and enum types in `GasChange`.
pub mod gas_change {
    /// Obtain all gas change reasons under deep mind repository:
    ///
    /// ```shell
    /// ack -ho 'GasChangeReason\(".*"\)' | grep -Eo '".*"' | sort | uniq
    /// ```
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Reason {
        Unknown = 0,
        /// REASON_CALL is the amount of gas that will be charged for a 'CALL' opcode executed by the EVM
        Call = 1,
        /// REASON_CALL_CODE is the amount of gas that will be charged for a 'CALLCODE' opcode executed by the EVM
        CallCode = 2,
        /// REASON_CALL_DATA_COPY is the amount of gas that will be charged for a 'CALLDATACOPY' opcode executed by the EVM
        CallDataCopy = 3,
        /// REASON_CODE_COPY is the amount of gas that will be charged for a 'CALLDATACOPY' opcode executed by the EVM
        CodeCopy = 4,
        /// REASON_CODE_STORAGE is the amount of gas that will be charged for code storage
        CodeStorage = 5,
        /// REASON_CONTRACT_CREATION is the amount of gas that will be charged for a 'CREATE' opcode executed by the EVM and for the gas
        /// burned for a CREATE, today controlled by EIP150 rules
        ContractCreation = 6,
        /// REASON_CONTRACT_CREATION2 is the amount of gas that will be charged for a 'CREATE2' opcode executed by the EVM and for the gas
        /// burned for a CREATE2, today controlled by EIP150 rules
        ContractCreation2 = 7,
        /// REASON_DELEGATE_CALL is the amount of gas that will be charged for a 'DELEGATECALL' opcode executed by the EVM
        DelegateCall = 8,
        /// REASON_EVENT_LOG is the amount of gas that will be charged for a 'LOG<N>' opcode executed by the EVM
        EventLog = 9,
        /// REASON_EXT_CODE_COPY is the amount of gas that will be charged for a 'LOG<N>' opcode executed by the EVM
        ExtCodeCopy = 10,
        /// REASON_FAILED_EXECUTION is the burning of the remaining gas when the execution failed without a revert
        FailedExecution = 11,
        /// REASON_INTRINSIC_GAS is the amount of gas that will be charged for the intrinsic cost of the transaction, there is
        /// always exactly one of those per transaction
        IntrinsicGas = 12,
        /// GasChangePrecompiledContract is the amount of gas that will be charged for a precompiled contract execution
        PrecompiledContract = 13,
        /// REASON_REFUND_AFTER_EXECUTION is the amount of gas that will be refunded to the caller after the execution of the call,
        /// if there is left over at the end of execution
        RefundAfterExecution = 14,
        /// REASON_RETURN is the amount of gas that will be charged for a 'RETURN' opcode executed by the EVM
        Return = 15,
        /// REASON_RETURN_DATA_COPY is the amount of gas that will be charged for a 'RETURNDATACOPY' opcode executed by the EVM
        ReturnDataCopy = 16,
        /// REASON_REVERT is the amount of gas that will be charged for a 'REVERT' opcode executed by the EVM
        Revert = 17,
        /// REASON_SELF_DESTRUCT is the amount of gas that will be charged for a 'SELFDESTRUCT' opcode executed by the EVM
        SelfDestruct = 18,
        /// REASON_STATIC_CALL is the amount of gas that will be charged for a 'STATICALL' opcode executed by the EVM
        StaticCall = 19,
        /// REASON_STATE_COLD_ACCESS is the amount of gas that will be charged for a cold storage access as controlled by EIP2929 rules
        ///
        /// Added in Berlin fork (Geth 1.10+)
        StateColdAccess = 20,
        /// REASON_TX_INITIAL_BALANCE is the initial balance for the call which will be equal to the gasLimit of the call
        ///
        /// Added as new tracing reason in Geth, available only on some chains
        TxInitialBalance = 21,
        /// REASON_TX_REFUNDS is the sum of all refunds which happened during the tx execution (e.g. storage slot being cleared)
        /// this generates an increase in gas. There is only one such gas change per transaction.
        ///
        /// Added as new tracing reason in Geth, available only on some chains
        TxRefunds = 22,
        /// REASON_TX_LEFT_OVER_RETURNED is the amount of gas left over at the end of transaction's execution that will be returned
        /// to the chain. This change will always be a negative change as we "drain" left over gas towards 0. If there was no gas
        /// left at the end of execution, no such even will be emitted. The returned gas's value in Wei is returned to caller.
        /// There is at most one of such gas change per transaction.
        ///
        /// Added as new tracing reason in Geth, available only on some chains
        TxLeftOverReturned = 23,
        /// REASON_CALL_INITIAL_BALANCE is the initial balance for the call which will be equal to the gasLimit of the call. There is only
        /// one such gas change per call.
        ///
        /// Added as new tracing reason in Geth, available only on some chains
        CallInitialBalance = 24,
        /// REASON_CALL_LEFT_OVER_RETURNED is the amount of gas left over that will be returned to the caller, this change will always
        /// be a negative change as we "drain" left over gas towards 0. If there was no gas left at the end of execution, no such even
        /// will be emitted.
        CallLeftOverReturned = 25,
    }
    impl Reason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Reason::Unknown => "REASON_UNKNOWN",
                Reason::Call => "REASON_CALL",
                Reason::CallCode => "REASON_CALL_CODE",
                Reason::CallDataCopy => "REASON_CALL_DATA_COPY",
                Reason::CodeCopy => "REASON_CODE_COPY",
                Reason::CodeStorage => "REASON_CODE_STORAGE",
                Reason::ContractCreation => "REASON_CONTRACT_CREATION",
                Reason::ContractCreation2 => "REASON_CONTRACT_CREATION2",
                Reason::DelegateCall => "REASON_DELEGATE_CALL",
                Reason::EventLog => "REASON_EVENT_LOG",
                Reason::ExtCodeCopy => "REASON_EXT_CODE_COPY",
                Reason::FailedExecution => "REASON_FAILED_EXECUTION",
                Reason::IntrinsicGas => "REASON_INTRINSIC_GAS",
                Reason::PrecompiledContract => "REASON_PRECOMPILED_CONTRACT",
                Reason::RefundAfterExecution => "REASON_REFUND_AFTER_EXECUTION",
                Reason::Return => "REASON_RETURN",
                Reason::ReturnDataCopy => "REASON_RETURN_DATA_COPY",
                Reason::Revert => "REASON_REVERT",
                Reason::SelfDestruct => "REASON_SELF_DESTRUCT",
                Reason::StaticCall => "REASON_STATIC_CALL",
                Reason::StateColdAccess => "REASON_STATE_COLD_ACCESS",
                Reason::TxInitialBalance => "REASON_TX_INITIAL_BALANCE",
                Reason::TxRefunds => "REASON_TX_REFUNDS",
                Reason::TxLeftOverReturned => "REASON_TX_LEFT_OVER_RETURNED",
                Reason::CallInitialBalance => "REASON_CALL_INITIAL_BALANCE",
                Reason::CallLeftOverReturned => "REASON_CALL_LEFT_OVER_RETURNED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "REASON_UNKNOWN" => Some(Self::Unknown),
                "REASON_CALL" => Some(Self::Call),
                "REASON_CALL_CODE" => Some(Self::CallCode),
                "REASON_CALL_DATA_COPY" => Some(Self::CallDataCopy),
                "REASON_CODE_COPY" => Some(Self::CodeCopy),
                "REASON_CODE_STORAGE" => Some(Self::CodeStorage),
                "REASON_CONTRACT_CREATION" => Some(Self::ContractCreation),
                "REASON_CONTRACT_CREATION2" => Some(Self::ContractCreation2),
                "REASON_DELEGATE_CALL" => Some(Self::DelegateCall),
                "REASON_EVENT_LOG" => Some(Self::EventLog),
                "REASON_EXT_CODE_COPY" => Some(Self::ExtCodeCopy),
                "REASON_FAILED_EXECUTION" => Some(Self::FailedExecution),
                "REASON_INTRINSIC_GAS" => Some(Self::IntrinsicGas),
                "REASON_PRECOMPILED_CONTRACT" => Some(Self::PrecompiledContract),
                "REASON_REFUND_AFTER_EXECUTION" => Some(Self::RefundAfterExecution),
                "REASON_RETURN" => Some(Self::Return),
                "REASON_RETURN_DATA_COPY" => Some(Self::ReturnDataCopy),
                "REASON_REVERT" => Some(Self::Revert),
                "REASON_SELF_DESTRUCT" => Some(Self::SelfDestruct),
                "REASON_STATIC_CALL" => Some(Self::StaticCall),
                "REASON_STATE_COLD_ACCESS" => Some(Self::StateColdAccess),
                "REASON_TX_INITIAL_BALANCE" => Some(Self::TxInitialBalance),
                "REASON_TX_REFUNDS" => Some(Self::TxRefunds),
                "REASON_TX_LEFT_OVER_RETURNED" => Some(Self::TxLeftOverReturned),
                "REASON_CALL_INITIAL_BALANCE" => Some(Self::CallInitialBalance),
                "REASON_CALL_LEFT_OVER_RETURNED" => Some(Self::CallLeftOverReturned),
                _ => None,
            }
        }
    }
}
/// HeaderOnlyBlock is used to optimally unpack the \[Block\] structure (note the
/// corresponding message number for the `header` field) while consuming less
/// memory, when only the `header` is desired.
///
/// WARN: this is a client-side optimization pattern and should be moved in the
/// consuming code.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct HeaderOnlyBlock {
    #[prost(message, optional, tag="5")]
    pub header: ::core::option::Option<BlockHeader>,
}
/// BlockWithRefs is a lightweight block, with traces and transactions
/// purged from the `block` within, and only.  It is used in transports
/// to pass block data around.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct BlockWithRefs {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub block: ::core::option::Option<Block>,
    #[prost(message, optional, tag="3")]
    pub transaction_trace_refs: ::core::option::Option<TransactionRefs>,
    #[prost(bool, tag="4")]
    pub irreversible: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct TransactionTraceWithBlockRef {
    #[prost(message, optional, tag="1")]
    pub trace: ::core::option::Option<TransactionTrace>,
    #[prost(message, optional, tag="2")]
    pub block_ref: ::core::option::Option<BlockRef>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct TransactionRefs {
    #[prost(bytes="vec", repeated, tag="1")]
    pub hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message,Serialize, Deserialize)]
pub struct BlockRef {
    #[prost(bytes="vec", tag="1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="2")]
    pub number: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionTraceStatus {
    Unknown = 0,
    Succeeded = 1,
    Failed = 2,
    Reverted = 3,
}
impl TransactionTraceStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TransactionTraceStatus::Unknown => "UNKNOWN",
            TransactionTraceStatus::Succeeded => "SUCCEEDED",
            TransactionTraceStatus::Failed => "FAILED",
            TransactionTraceStatus::Reverted => "REVERTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "SUCCEEDED" => Some(Self::Succeeded),
            "FAILED" => Some(Self::Failed),
            "REVERTED" => Some(Self::Reverted),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CallType {
    Unspecified = 0,
    /// direct? what's the name for `Call` alone?
    Call = 1,
    Callcode = 2,
    Delegate = 3,
    Static = 4,
    /// create2 ? any other form of calls?
    Create = 5,
}
impl CallType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CallType::Unspecified => "UNSPECIFIED",
            CallType::Call => "CALL",
            CallType::Callcode => "CALLCODE",
            CallType::Delegate => "DELEGATE",
            CallType::Static => "STATIC",
            CallType::Create => "CREATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "CALL" => Some(Self::Call),
            "CALLCODE" => Some(Self::Callcode),
            "DELEGATE" => Some(Self::Delegate),
            "STATIC" => Some(Self::Static),
            "CREATE" => Some(Self::Create),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
