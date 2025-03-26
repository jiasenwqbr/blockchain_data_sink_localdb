--- block 
create table IF NOT EXISTS solana_block_all (id text primary key, data text);

--- block head
create table IF NOT EXISTS solana_block_head (
    id text primary key,
    previous_blockhash text,
    blockhash text,
    parent_slot bigint,
    block_time bigint,
    block_height bigint,
    block_number bigint,
    slot bigint
);

--- block transaction id=block_number+transaction_index+signature_index
create table IF NOT EXISTS solana_block_transaction_signature (
    id text primary key,
    block_number bigint,
    signature text,
    transaction_index integer,
    signature_index integer
);

--- block transaction message  id=block_number+transaction_index+
create table IF NOT EXISTS solana_block_transaction_message (
    id text primary key,
    block_number bigint,
    transaction_index integer,
    header_num_required_signatures integer,
    header_num_readonly_signed_accounts integer,
    header_num_readonly_unsigned_accounts,
    recent_blockhash text,
    versioned BOOLEAN
);

--- block transaction message account_keys id=block_number+transaction_index+account_keys_index
create table IF NOT EXISTS solana_block_transaction_message_account_key (
    id text primary key,
    block_number bigint,
    transaction_index integer,
    account_key text,
    account_key_index integer
);

--- block transaction message Instruction id=block_number+transaction_index+instruction_index
create table IF NOT EXISTS solana_block_transaction_message_instruction (
    id text primary key,
    block_number bigint,
    transaction_index integer,
    program_id_index integer,
    instruction_accounts text,
    instruction_data text,
    instruction_index integer
);

--- block transaction message address_table_lookups id=block_number+transaction_index+iaddress_table_lookups_index
create table IF NOT EXISTS solana_block_transaction_message_address_table_lookups (
    id text primary key,
    block_number bigint,
    transaction_index integer,
    account_key text,
    writable_indexes text,
    readonly_indexes text,
    address_table_lookups_index integer
);


--- block TransactionStatusMeta   id=block_number+Transaction_index
create table IF NOT EXISTS solana_block_transaction_status_meta (
    id text primary key,
    block_number bigint,
    transaction_index integer,
    fee bigint,
    pre_balances bigint,
    post_balances bigint,
    inner_instructions_none  boolean,
    log_messages_none boolean,
    loaded_writable_addresses text,
    loaded_readonly_addresses text,
    return_data_none boolean,
    return_data_program_id text,
    return_data_data text
);

--- block TransactionStatusMeta  InnerInstructions  id=block_number+Transaction_index+InnerInstructions_index+instruction_index
create table IF NOT EXISTS solana_block_transaction_status_meta_inner_instruction (
    id text primary key,
    block_number bigint,
    transaction_index integer,
    inner_instructions_index integer,
    inner_instruction_index integer,
    program_id_index integer,
    accounts text,
    instruction_data text,
    stack_height integer
);

--- block TransactionStatusMeta  token_balances  id=block_number+Transaction_index+token_balances_index_pre/postflag   0 pre 1 post
create table IF NOT EXISTS solana_block_transaction_status_meta_pre_token_balances (
    id text primary key,
    block_number bigint,
    transaction_index integer,
    pre_post_flag integer,
    balances_index integer,
    account_index  integer,
    mint text,
    ui_token_amount_ui_amount bigint,
    ui_token_amount_decimals integer,
    ui_token_amount_amount integer,
    ui_token_amount_ui_amount_string text,
    owner text,
    program_id text
);

--- block TransactionStatusMeta  rewards  id=block_number+Transaction_index+rewards_index
create table IF NOT EXISTS solana_block_transaction_status_meta_rewards (
    id text primary key,
    block_number bigint,
    transaction_index integer,
    reward_index integer,
    pubkey text,
    lamports bigint,
    post_balance bigint,
    reward_type integer,
    commission text
);

--- block   rewards  id=block_number+rewards_index
create table IF NOT EXISTS solana_block_rewards (
    id text primary key,
    block_number bigint,
    reward_index integer,
    pubkey text,
    lamports bigint,
    post_balance bigint,
    reward_type integer,
    commission text
);


