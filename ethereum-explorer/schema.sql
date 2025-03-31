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
    base_fee_per_gas text,
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
create table IF NOT EXISTS ethereum_block_transaction_access_list_storage_keys (
    id text primary key,
    block_number bigint,
    parent_id text,
    parent_table text,
    access_index bigint,
    key_index bigint,
    storage_key text
);


--- block transaction receipt   id = block_number+transaction_index
create table IF NOT EXISTS ethereum_block_transaction_receipt (
    id text primary key,
    block_number bigint,
    parent_id text,
    transaction_index bigint,
    parent_table text,
    state_root text,

    cumulative_gas_used bigint,
    logs_bloom text,
    blob_gas_used bigint,
    blob_gas_price text
);



--- block transaction receipt  logs  id = block_number+transaction_index_log_index
create table IF NOT EXISTS ethereum_block_transaction_receipt_logs (
    id text primary key,
    block_number bigint,
    parent_id text,
    parent_table text,
    log_index bigint,
    address text,

    data text,
    index bigint,
    block_index bigint,
    ordinal bigint
);


--- block transaction receipt  logs topics  id = block_number+transaction_index_log_index+topic_index
create table IF NOT EXISTS ethereum_block_transaction_receipt_logs_topics (
    id text primary key,
    block_number bigint,
    parent_id text,
    parent_table text,
    log_index bigint,
    topic_index bigint,
    topic text
);

-- block code_changes         id = blocknumber+code_changes_index
create table IF NOT EXISTS ethereum_block_code_changes (
    id text primary key,
    block_number bigint,
    parent_id text,
    parent_table text,
    change_index bigint,
    address text,
    old_code text,
    old_hash text,
    new_code text,
    new_hash text,
    ordinal bigint
);

-- block balance_changes         id = blocknumber+balance_changes_index
create table IF NOT EXISTS ethereum_block_balance_changes (
    id text primary key,
    block_number bigint,
    parent_id text,
    parent_table text,
    change_index bigint,
    address text,
    old_value text,
    new_value text,
    reason text,
    ordinal bigint
);



-- block system_calls id = blocknumber+system_calls_index
create table IF NOT EXISTS ethereum_block_system_calls (
    id text primary key,
    block_number bigint,
    parent_id text,
    parent_table text,
    system_call_index bigint,
    index bigint,
    parent_index bigint,
    depth bigint,
    call_type bigint,
    caller text,
    address text,
    system_call_value text,
    gas_limit bigint,
    gas_consumed bigint,
    return_data text,
    input text,
    executed_code bool,
    suicide bool,
    status_failed bool,
    status_reverted bool,
    failure_reason text,
    state_reverted bool,
    begin_ordinal bigint,
    end_ordinal bigint
);

-- block system_calls storage_change id = blocknumber+system_calls_index+storage_change_index
create table IF NOT EXISTS ethereum_block_system_calls_storage_changes (
    id text primary key,
    block_number bigint,
    parent_id text,
    parent_table text,
    system_call_index bigint,
    change_index bigint,
    address text,
    key text,
    old_value text,
    new_value text,
    ordinal bigint
);


-- block system_calls balance_change id = blocknumber+system_calls_index+balance_change_index
create table IF NOT EXISTS ethereum_block_system_calls_balance_changes(
    id text primary key,
    block_number bigint,
    parent_id text,
    parent_table text,
    system_call_index bigint,
    change_index bigint,
    address text,
    old_value text,
    new_value text,
    reason text,
    ordinal bigint
);



-- block system_calls nonce_change  id = blocknumber+system_calls_index+nonce_change_index
create table IF NOT EXISTS ethereum_block_system_calls_nonce_changes(
    id text primary key,
    block_number bigint,
    parent_id text,
    parent_table text,
    system_call_index bigint,
    change_index bigint,
    address text,
    old_value bigint,
    new_value bigint,
    ordinal bigint
);


-- block system_calls  code_change  id = blocknumber+system_calls_index+code_change_index
create table IF NOT EXISTS ethereum_block_system_calls_code_changes(
    id text primary key,
    block_number bigint,
    parent_id text,
    parent_table text,
    system_call_index bigint,
    change_index bigint,
    address text,
    old_code text,
    old_value text,
    new_code text,
    new_value text,
    ordinal bigint
);


-- block system_calls gas_change    id = blocknumber+system_calls_index+gas_change_index
create table IF NOT EXISTS ethereum_block_system_calls_gas_changes(
    id text primary key,
    block_number bigint,
    parent_id text,
    parent_table text,
    system_call_index bigint,
    change_index bigint,
    address text,
    old_value text,
    new_value text,
    reason text,
    ordinal bigint
);


-- block system_calls account_creations id = block_number+ account_careation_index

create table IF NOT EXISTS ethereum_block_system_calls_account_creations(
    id text primary key,
    block_number bigint,
    parent_id text,
    parent_table text,
    system_call_index bigint,
    account_index bigint,
    account text,
    ordinal bigint
);






































