--- block 
create table IF NOT EXISTS eth_block (id text primary key, data text);
--- block      id = block_number
create table IF NOT EXISTS ethereum_block (
    id text primary key,
    block_hash text,
    block_number bigint,
    block_size bigint,
    detail_level bigint,
    ver bigint
);

--- block header            id = block_number
create table IF NOT EXISTS ethereum_block_header (
    id text primary key,
    block_number bigint,
    parent_id text,
    parent_table text,
    parent_hash text,
    uncle_hash text,
    coinbase text,
    state_root text,
    transactions_root text,
    receipt_root text,
    logs_bloom text,
    difficulty text,
    total_difficulty text,
    header_number bigint,
    gas_limit bigint,
    gas_used bigint,
    header_timestamp bigint,
    extra_data text,
    mix_hash text,
    nonce bigint,
    header_hash text,
    base_fee_per_gas bigint,
    withdrawals_root text,
    tx_dependency text,
    blob_gas_used bigint,
    excess_blob_gas bigint,
    parent_beacon_root text
);

--- block header            id = block_number+header_index
create table IF NOT EXISTS ethereum_block_uncles (
    id text primary key,
    block_number bigint,
    parent_id text,
    parent_table text,
    parent_hash text,
    uncle_hash text,
    coinbase text,
    state_root text,
    transactions_root text,
    receipt_root text,
    logs_bloom text,
    difficulty text,
    total_difficulty text,
    header_number bigint,
    gas_limit bigint,
    gas_used bigint,
    header_timestamp bigint,
    extra_data text,
    mix_hash text,
    nonce bigint,
    header_hash text,
    base_fee_per_gas bigint,
    withdrawals_root text,
    tx_dependency text,
    blob_gas_used bigint,
    excess_blob_gas bigint,
    parent_beacon_root text
);

--- block transaction       id=block_number+transaction_index
create table IF NOT EXISTS ethereum_block_transaction (
    id text primary key,
    block_number bigint,
    parent_id text,
    parent_table text,
    transaction_to text,
    nonce bigint,
    gas_price text,
    gas_limit bigint,
    transaction_value text,
    input text,
    v text,
    r text,
    s text,
    gas_used bigint,
    transaction_type bigint,
    max_fee_per_gas text,
    max_priority_fee_per_gas text,
    transaction_index bigint,
    transaction_hash  text,
    transaction_from text,
    return_data text,
    public_key text,
    begin_ordinal bigint,
    end_ordinal bigint,
    transaction_status bigint,
    blob_gas text,
    blob_gas_fee_cap text,
    blob_hashes text
);

--- block transaction access_list   id = block_number+transaction_index+access_index
create table IF NOT EXISTS ethereum_block_transaction_access_list (
    id text primary key,
    block_number bigint,
    parent_id text,
    parent_table text,
    access_index bigint,
    access_address text
);

--- block transaction access_list storagekeys   id = block_number+transaction_index+access_index+key_index
create table IF NOT EXISTS ethereum_block_transaction_access_list (
    id text primary key,
    block_number bigint,
    parent_id text,
    parent_table text,
    access_index bigint,
    key_index bigint,
    storage_key text
);

















