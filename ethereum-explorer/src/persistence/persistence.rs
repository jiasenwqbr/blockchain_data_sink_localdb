use std::collections::HashMap;
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use substreams_ethereum::pb::eth::v2::Block;

use crate::pb::eth::transaction;


pub fn save_ethereum_block(block: Block, database_changes: &mut DatabaseChanges) {
    let block_number = block.number;
    let block_hash = hex::encode(block.hash);
    let block_size = block.size;
    let detail_level = block.detail_level;
    let ver = block.ver;

    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), block_number.to_string());

    database_changes.push_change_composite("ethereum_block", keys, 1, Operation::Create)
    .change("block_number", (None,block_number))
    .change("block_hash", (None,block_hash))
    .change("block_size", (None,block_size))
    .change("detail_level", (None,detail_level))
    .change("ver", (None,ver));

    // block header
    let header = block.header;
    match header {
        Some(head) => {
            let parent_hash = hex::encode(head.parent_hash);
            let uncle_hash = hex::encode(head.uncle_hash);
            let coinbase = hex::encode(head.coinbase);
            let state_root = hex::encode(head.state_root);
            let transactions_root = hex::encode(head.transactions_root);
            let receipt_root = hex::encode(head.receipt_root);
            let logs_bloom = hex::encode(head.logs_bloom);
            let difficulty = hex::encode(head.difficulty.unwrap().bytes);
            let total_difficulty = hex::encode(head.total_difficulty.unwrap_or_default().bytes);
            let header_number = head.number;
            let gas_limit = head.gas_limit;
            let gas_used = head.gas_used;
            let timestm = head.timestamp.unwrap_or_default();
            let header_timestamp = timestm.seconds as u64 * 1000 + (timestm.nanos as u64/1_000_000);
            let extra_data = hex::encode(head.extra_data);
            let mix_hash = hex::encode(head.mix_hash);
            let nonce = head.nonce;
            let header_hash = hex::encode(head.hash);
            let base_fee_per_gas = hex::encode(head.base_fee_per_gas.unwrap_or_default().bytes);
            let withdrawals_root = hex::encode(head.withdrawals_root);
            let blob_gas_used = head.blob_gas_used.unwrap_or_default();
            let excess_blob_gas = head.excess_blob_gas.unwrap_or_default();
            let parent_beacon_root = hex::encode(head.parent_beacon_root);
            let parent_id = block_number.to_string();
            let parent_table = "ethereum_block".to_string();
            let id = block_number.to_string();

            save_ethereum_block_header(
                id,
                block_number,
                parent_id,
                parent_table,
                parent_hash,
                uncle_hash,
                coinbase,
                state_root,
                transactions_root,
                receipt_root,
                logs_bloom,
                difficulty,
                total_difficulty,
                header_number,
                gas_limit,
                gas_used,
                header_timestamp,
                extra_data,
                mix_hash,
                nonce,
                header_hash,
                base_fee_per_gas,
                withdrawals_root,
                blob_gas_used,
                excess_blob_gas,
                parent_beacon_root,
                database_changes
            );

        },
        None => {}
    }

    // block uncles
    let uncles = block.uncles;
    for (header_index,head) in uncles.iter().enumerate(){
        let parent_hash = hex::encode(&head.parent_hash);
            let uncle_hash = hex::encode(&head.uncle_hash);
            let coinbase = hex::encode(&head.coinbase);
            let state_root = hex::encode(&head.state_root);
            let transactions_root = hex::encode(&head.transactions_root);
            let receipt_root = hex::encode(&head.receipt_root);
            let logs_bloom = hex::encode(&head.logs_bloom);
            let difficulty = hex::encode(&head.difficulty.clone().unwrap().bytes);
            let total_difficulty = hex::encode(&head.total_difficulty.clone().unwrap_or_default().bytes);
            let header_number = head.number;
            let gas_limit = head.gas_limit;
            let gas_used = head.gas_used;
            let timestm = &head.timestamp.clone().unwrap_or_default();
            let header_timestamp = timestm.seconds as u64 * 1000 + (timestm.nanos as u64/1_000_000);
            let extra_data = hex::encode(&head.extra_data);
            let mix_hash = hex::encode(&head.mix_hash);
            let nonce = head.nonce;
            let header_hash = hex::encode(&head.hash);
            let base_fee_per_gas = hex::encode(&head.base_fee_per_gas.clone().unwrap_or_default().bytes);
            let withdrawals_root = hex::encode(&head.withdrawals_root);
            let blob_gas_used = head.blob_gas_used.unwrap_or_default();
            let excess_blob_gas = head.excess_blob_gas.unwrap_or_default();
            let parent_beacon_root = hex::encode(&head.parent_beacon_root);
            let parent_id = block_number.to_string();
            let parent_table = "ethereum_block".to_string();
            let id = format!("{}_{}",block_number.to_string(),header_index);
            save_ethereum_block_uncles(
                id,
                block_number,
                parent_id,
                parent_table,
                parent_hash,
                uncle_hash,
                coinbase,
                state_root,
                transactions_root,
                receipt_root,
                logs_bloom,
                difficulty,
                total_difficulty,
                header_number,
                gas_limit,
                gas_used,
                header_timestamp,
                extra_data,
                mix_hash,
                nonce,
                header_hash,
                base_fee_per_gas,
                withdrawals_root,
                blob_gas_used,
                excess_blob_gas,
                parent_beacon_root,
                database_changes
            );
    }

    // block transaction
    let transactions: Vec<substreams_ethereum::pb::eth::v2::TransactionTrace> = block.transaction_traces;
    for (transaction_index,transaction) in transactions.iter().enumerate() {
        let id = format!("{}_{}",block_number,transaction_index);
        let parent_id = block_number.to_string();
        let parent_table = "ethereum_block".to_string();
        let transaction_to = hex::encode(&transaction.to);
        let nonce = transaction.nonce;
        let gas_price = hex::encode(transaction.gas_price.clone().unwrap().bytes);
        let gas_limit = transaction.gas_limit;
        let transaction_value = hex::encode(transaction.value.clone().unwrap().bytes);
        let input = hex::encode( &transaction.input);
        let v = hex::encode(&transaction.v);
        let r = hex::encode(&transaction.r);
        let s = hex::encode(&transaction.s);
        let gas_used = transaction.gas_used;
        let transaction_type = transaction.r#type;
        let max_fee_per_gas = hex::encode(transaction.max_fee_per_gas.clone().unwrap().bytes);
        let max_priority_fee_per_gas = hex::encode(transaction.max_priority_fee_per_gas.clone().unwrap().bytes);
        let transaction_index = transaction.index;
        let transaction_hash = hex::encode(&transaction.hash);
        let transaction_from = hex::encode(&transaction.from);
        let return_data = hex::encode(&transaction.return_data);
        let public_key = hex::encode(&transaction.public_key);
        let begin_ordinal = transaction.begin_ordinal;
        let end_ordinal = transaction.end_ordinal;
        let transaction_status = transaction.status;
        let blob_gas = transaction.blob_gas.unwrap_or_default();
        let blob_gas_fee_cap = hex::encode(transaction.blob_gas_fee_cap.clone().unwrap().bytes);
        
        save_ethereum_block_transaction(
            id,
            block_number,
            parent_id,
            parent_table,
            transaction_to,
            nonce,
            gas_price,
            gas_limit,
            transaction_value,
            input,
            v,r,s,
            gas_used,
            transaction_type,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            transaction_index,
            transaction_hash,
            transaction_from,
            return_data,
            public_key,
            begin_ordinal,
            end_ordinal,
            transaction_status,
            blob_gas,
            blob_gas_fee_cap,
            database_changes
        );

        // block transaction access_list 
        let access_list = &transaction.access_list;
        for (access_index,access) in access_list.iter().enumerate(){
            let access_address = hex::encode(&access.address);
            let id = format!("{}_{}_{}",block_number,transaction_index,access_index);
            let parent_id = format!("{}_{}",block_number,transaction_index);
            let parent_table = "ethereum_block_transaction".to_string();

            save_ethereum_block_transaction_access_list(
                id,
                block_number,
                parent_id,
                parent_table,
                access_index,
                access_address,
                database_changes
            );

            let storagekeys = &access.storage_keys;
            for (key_index,storage) in storagekeys.iter().enumerate(){
                let storage_key = hex::encode(storage);
                let id: String = format!("{}_{}_{}_{}",block_number,transaction_index,access_index,key_index);
                let parent_id = format!("{}_{}_{}",block_number,transaction_index,access_index);
                let parent_table = "ethereum_block_transaction_access_list".to_string();
                save_ethereum_block_transaction_access_list_storage_keys(
                    id,
                    block_number,
                    parent_id,
                    parent_table,
                    access_index,
                    key_index,
                    storage_key,
                    database_changes
                );
            }




        }

        // block transaction receipt
        let receipt = &transaction.receipt;
        match receipt {
            Some(transactionReceipt) => {
                let state_root = hex::encode(&transactionReceipt.state_root);
                let cumulative_gas_used = transactionReceipt.cumulative_gas_used;
                let logs_bloom = hex::encode(&transactionReceipt.logs_bloom);
                let blob_gas_used = transactionReceipt.blob_gas_used.unwrap_or_default();
                let blob_gas_price = hex::encode(transactionReceipt.blob_gas_price.clone().unwrap_or_default().bytes);

            },
            None => {}
        }





    }


}

fn save_ethereum_block_transaction_access_list_storage_keys(
    id : String,
    block_number : u64,
    parent_id : String,
    parent_table : String,
    access_index : usize,
    key_index : usize,
    storage_key : String,
    changes : &mut DatabaseChanges
){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id);
    changes.push_change_composite("ethereum_block_header", keys, 1, Operation::Create)
    .change("block_number", (None,block_number))
    .change("parent_id", (None, parent_id))
    .change("parent_table", (None, parent_table))
    .change("access_index", (None,access_index as u64))
    .change("key_index", (None,key_index as u64))
    .change("storage_key", (None,storage_key));
}


fn save_ethereum_block_transaction_access_list(
    id : String,
    block_number : u64,
    parent_id : String,
    parent_table : String,
    access_index : usize,
    access_address : String,
    changes : &mut DatabaseChanges
){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id);
    changes.push_change_composite("ethereum_block_header", keys, 1, Operation::Create)
    .change("block_number", (None,block_number))
    .change("parent_id", (None, parent_id))
    .change("parent_table", (None, parent_table))
    .change("access_index", (None,access_index as u64))
    .change("access_address", (None,access_address));
}


fn save_ethereum_block_header(
    id : String,
    block_number:u64,
    parent_id:String,
    parent_table:String,
    parent_hash:String,
    uncle_hash:String,
    coinbase:String,
    state_root:String,
    transactions_root:String,
    receipt_root:String,
    logs_bloom:String,
    difficulty:String,
    total_difficulty:String,
    header_number : u64,
    gas_limit:u64,
    gas_used:u64,
    header_timestamp:u64,
    extra_data:String,
    mix_hash:String,
    nonce:u64,
    header_hash:String,
    base_fee_per_gas:String,
    withdrawals_root:String,
    blob_gas_used:u64,
    excess_blob_gas:u64,
    parent_beacon_root:String,
    changes:&mut DatabaseChanges
){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id);
    changes.push_change_composite("ethereum_block_header", keys, 1, Operation::Create)
    .change("block_number", (None,block_number))
    .change("parent_id", (None, parent_id))
    .change("parent_table", (None, parent_table))
    .change("parent_hash", (None, parent_hash))
    .change("uncle_hash", (None, uncle_hash))
    .change("coinbase", (None, coinbase))
    .change("state_root", (None, state_root))
    .change("transactions_root", (None, transactions_root))
    .change("receipt_root", (None, receipt_root))
    .change("logs_bloom", (None, logs_bloom))
    .change("difficulty", (None, difficulty))
    .change("total_difficulty", (None, total_difficulty))
    .change("header_number", (None, header_number))
    .change("gas_limit", (None, gas_limit))
    .change("gas_used", (None, gas_used))
    .change("header_timestamp", (None, header_timestamp))
    .change("extra_data", (None, extra_data))
    .change("mix_hash", (None, mix_hash))
    .change("nonce", (None, nonce))
    .change("header_hash", (None, header_hash))
    .change("base_fee_per_gas", (None, base_fee_per_gas))
    .change("withdrawals_root", (None, withdrawals_root))
    .change("blob_gas_used", (None, blob_gas_used))
    .change("excess_blob_gas", (None, excess_blob_gas))
    .change("parent_beacon_root", (None, parent_beacon_root));
}

fn save_ethereum_block_uncles(
    id : String,
    block_number:u64,
    parent_id:String,
    parent_table:String,
    parent_hash:String,
    uncle_hash:String,
    coinbase:String,
    state_root:String,
    transactions_root:String,
    receipt_root:String,
    logs_bloom:String,
    difficulty:String,
    total_difficulty:String,
    header_number : u64,
    gas_limit:u64,
    gas_used:u64,
    header_timestamp:u64,
    extra_data:String,
    mix_hash:String,
    nonce:u64,
    header_hash:String,
    base_fee_per_gas:String,
    withdrawals_root:String,
    blob_gas_used:u64,
    excess_blob_gas:u64,
    parent_beacon_root:String,
    changes:&mut DatabaseChanges
){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id);
    changes.push_change_composite("ethereum_block_uncles", keys, 1, Operation::Create)
    .change("block_number", (None,block_number))
    .change("parent_id", (None, parent_id))
    .change("parent_table", (None, parent_table))
    .change("parent_hash", (None, parent_hash))
    .change("uncle_hash", (None, uncle_hash))
    .change("coinbase", (None, coinbase))
    .change("state_root", (None, state_root))
    .change("transactions_root", (None, transactions_root))
    .change("receipt_root", (None, receipt_root))
    .change("logs_bloom", (None, logs_bloom))
    .change("difficulty", (None, difficulty))
    .change("total_difficulty", (None, total_difficulty))
    .change("header_number", (None, header_number))
    .change("gas_limit", (None, gas_limit))
    .change("gas_used", (None, gas_used))
    .change("header_timestamp", (None, header_timestamp))
    .change("extra_data", (None, extra_data))
    .change("mix_hash", (None, mix_hash))
    .change("nonce", (None, nonce))
    .change("header_hash", (None, header_hash))
    .change("base_fee_per_gas", (None, base_fee_per_gas))
    .change("withdrawals_root", (None, withdrawals_root))
    .change("blob_gas_used", (None, blob_gas_used))
    .change("excess_blob_gas", (None, excess_blob_gas))
    .change("parent_beacon_root", (None, parent_beacon_root));
}


fn save_ethereum_block_transaction(
    id : String,
    block_number:u64,
    parent_id : String,
    parent_table : String,
    transaction_to : String,
    nonce : u64,
    gas_price : String,
    gas_limit : u64,
    transaction_value : String,
    input : String,
    v : String,
    r : String,
    s : String,
    gas_used : u64,
    transaction_type : i32,
    max_fee_per_gas : String,
    max_priority_fee_per_gas : String,
    transaction_index : u32,
    transaction_hash : String,
    transaction_from : String,
    return_data : String,
    public_key : String,
    begin_ordinal : u64 ,
    end_ordinal : u64,
    transaction_status : i32,
    blob_gas : u64,
    blob_gas_fee_cap : String,
    changes : &mut DatabaseChanges
){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id);
    changes.push_change_composite("ethereum_block_uncles", keys, 1, Operation::Create)
    .change("block_number", (None,block_number))
    .change("parent_id", (None, parent_id))
        .change("parent_table", (None, parent_table))
        .change("transaction_to", (None, transaction_to))
        .change("nonce", (None, nonce))
        .change("gas_price", (None, gas_price))
        .change("gas_limit", (None, gas_limit))
        .change("transaction_value", (None, transaction_value))
        .change("input", (None, input))
        .change("v", (None, v))
        .change("r", (None, r))
        .change("s", (None, s))
        .change("gas_used", (None, gas_used))
        .change("transaction_type", (None, transaction_type))
        .change("max_fee_per_gas", (None, max_fee_per_gas))
        .change("max_priority_fee_per_gas", (None, max_priority_fee_per_gas))
        .change("transaction_index", (None, transaction_index))
        .change("transaction_hash", (None, transaction_hash))
        .change("transaction_from", (None, transaction_from))
        .change("return_data", (None, return_data))
        .change("public_key", (None, public_key))
        .change("begin_ordinal", (None, begin_ordinal))
        .change("end_ordinal", (None, end_ordinal))
        .change("transaction_status", (None, transaction_status))
        .change("blob_gas", (None, blob_gas))
        .change("blob_gas_fee_cap", (None, blob_gas_fee_cap));
    ;
    
}