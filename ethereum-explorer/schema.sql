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
    
);
