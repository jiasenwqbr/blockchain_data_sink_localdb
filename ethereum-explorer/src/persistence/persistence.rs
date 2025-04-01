use std::{collections::HashMap, i32};
use num_bigint::BigInt;
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use substreams_ethereum::pb::eth::v2::Block;

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

    // save_block(block,database_changes);

    // block header
    let header: Option<substreams_ethereum::pb::eth::v2::BlockHeader> = block.header;
    match header {
        Some(head) => {
            let parent_hash = hex::encode(head.parent_hash);
            let uncle_hash = hex::encode(head.uncle_hash);
            let coinbase = hex::encode(head.coinbase);
            let state_root = hex::encode(head.state_root);
            let transactions_root = hex::encode(head.transactions_root);
            let receipt_root = hex::encode(head.receipt_root);
            let logs_bloom = hex::encode(head.logs_bloom);
            let difficulty = match head.total_difficulty.clone() {
                Some(val) => BigInt::from_bytes_be(num_bigint::Sign::Plus,&val.bytes).to_string(),None => String::new()
            };
            //hex::encode(head.difficulty.unwrap().bytes);
            let total_difficulty = match head.total_difficulty.clone() {
                Some(val) => BigInt::from_bytes_be(num_bigint::Sign::Plus,&val.bytes).to_string(),None => String::new()
            };
            // hex::encode(head.total_difficulty.unwrap_or_default().bytes);
            let header_number = head.number;
            let gas_limit = head.gas_limit;
            let gas_used = head.gas_used;
            let timestm = head.timestamp.unwrap_or_default();
            let header_timestamp = timestm.seconds as u64 * 1000 + (timestm.nanos as u64/1_000_000);
            let extra_data = hex::encode(head.extra_data);
            let mix_hash = hex::encode(head.mix_hash);
            let nonce = head.nonce;
            let header_hash = hex::encode(head.hash);
            let base_fee_per_gas = match  head.base_fee_per_gas.clone() {
                Some(val) => BigInt::from_bytes_be(num_bigint::Sign::Plus,&val.bytes).to_string(),None => String::new()
            };
            
            // hex::encode(head.base_fee_per_gas.unwrap_or_default().bytes);
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
        let gas_price = match transaction.gas_price.clone() {
            Some(val) => BigInt::from_bytes_be(num_bigint::Sign::Plus,&val.bytes).to_string(),None => String::new()
        };
        let gas_limit = transaction.gas_limit;
        let transaction_value = match transaction.value.clone() {
            Some(val) => BigInt::from_bytes_be(num_bigint::Sign::Plus,&val.bytes).to_string(),None => String::new()
        };
        let input = hex::encode( &transaction.input);
        let v = hex::encode(&transaction.v);
        let r = hex::encode(&transaction.r);
        let s = hex::encode(&transaction.s);
        let gas_used = transaction.gas_used;
        let transaction_type = transaction.r#type;
        let max_fee_per_gas = match transaction.max_fee_per_gas.clone() {
            Some(val) => BigInt::from_bytes_be(num_bigint::Sign::Plus,&val.bytes).to_string(),None => String::new()
        };
        let max_priority_fee_per_gas = match transaction.max_priority_fee_per_gas.clone() {
            Some(val) => BigInt::from_bytes_be(num_bigint::Sign::Plus,&val.bytes).to_string(),None => String::new()
        };
        let transaction_ind = transaction.index;
        let transaction_hash = hex::encode(&transaction.hash);
        let transaction_from = hex::encode(&transaction.from);
        let return_data = hex::encode(&transaction.return_data);
        let public_key = hex::encode(&transaction.public_key);
        let begin_ordinal = transaction.begin_ordinal;
        let end_ordinal = transaction.end_ordinal;
        let transaction_status = transaction.status;
        let blob_gas = transaction.blob_gas.unwrap_or_default();
        let blob_gas_fee_cap: String = match transaction.blob_gas_fee_cap.clone() {
            Some(val) => hex::encode(val.bytes),None => String::new(),
        };

        // hex::encode(transaction.blob_gas_fee_cap.clone().unwrap_or_default().bytes);
        
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
            transaction_ind,
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
            Some(transaction_receipt) => {
                let state_root = hex::encode(&transaction_receipt.state_root);
                let cumulative_gas_used = transaction_receipt.cumulative_gas_used;
                let logs_bloom = hex::encode(&transaction_receipt.logs_bloom);
                let blob_gas_used = transaction_receipt.blob_gas_used.unwrap_or_default();
                let blob_gas_price = match transaction_receipt.blob_gas_price.clone() {
                    Some(val) => BigInt::from_bytes_be(num_bigint::Sign::Plus,&val.bytes).to_string(),None => String::new()
                };
                let receipt_id = format!("{}_{}",block_number,transaction_index);
                let parent_id: String = block_number.to_string();
                let parent_table = "ethereum_block_transaction".to_string();
                save_ethereum_block_transaction_receipt(
                    receipt_id,
                    block_number,
                    parent_id,
                    parent_table,
                    transaction_index,
                    state_root,
                    cumulative_gas_used,
                    logs_bloom,
                    blob_gas_used,
                    blob_gas_price,
                    database_changes
                );

                let logs = &transaction_receipt.logs;
                for (log_index, log) in logs.iter().enumerate(){
                    let address = hex::encode(&log.address);
                    
                    let data = hex::encode(&log.data);
                    let index = &log.index;
                    let block_index = &log.block_index;
                    let ordinal = &log.ordinal;
                    let id = format!("{}_{}_{}",block_number,transaction_index,log_index);
                    let parent_id: String = format!("{}_{}",block_number,transaction_index);
                    let parent_table = "ethereum_block_transaction_receipt".to_string();
                    save_ethereum_block_transaction_receipt_logs(
                        id,
                        block_number,
                        parent_id,
                        parent_table,
                        log_index,
                        address,
                        data,
                        index,
                        block_index,
                        ordinal,
                        database_changes
                    );

                    let topics = &log.topics;
                    for (topic_index,topic) in topics.iter().enumerate(){
                        let tp = hex::encode(topic);
                        
                        let id = format!("{}_{}_{}_{}",block_number,transaction_index,log_index,topic_index);
                        let parent_id: String = format!("{}_{}_{}",block_number,transaction_index,log_index);
                        let parent_table = "ethereum_block_transaction_receipt_logs".to_string();

                        save_ethereum_block_transaction_receipt_logs_topics(
                            id,
                            block_number,
                            parent_id,
                            parent_table,
                            log_index,
                            topic_index,
                            tp,
                            database_changes
                        );

                    }
                }


            },
            None => {}
        }
    }

    // block balancechanges
    let balance_changes = block.balance_changes;
    for (balance_change_index,balance_change) in balance_changes.iter().enumerate(){
        let id = format!("{}_{}",block_number,balance_change_index);
        let parent_id = format!("{}",block_number);
        let parent_table = "ethereum_block".to_string();
        let address = hex::encode(&balance_change.address);
        let old_value = match &balance_change.old_value.clone() {
            Some(val) => BigInt::from_bytes_be(num_bigint::Sign::Plus,&val.bytes).to_string(),None => String::new()
        };
        let new_value = match &balance_change.new_value.clone() {
            Some(val) => BigInt::from_bytes_be(num_bigint::Sign::Plus,&val.bytes).to_string(),None => String::new()
        };
        let reason = &balance_change.reason;
        let ordinal = &balance_change.ordinal;
        save_ethereum_block_balance_changes(
            id,
            block_number,
            parent_id,
            parent_table,
            address,
            old_value,
            new_value,
            reason,
            ordinal,
            database_changes
        );
    }
    // block code_changes
    let code_changes = block.code_changes;
    for (code_change_index,code_change) in code_changes.iter().enumerate(){
        let id = format!("{}_{}",block_number,code_change_index);
        let parent_id = format!("{}",block_number);
        let parent_table = "ethereum_block".to_string();
        let address = hex::encode(&code_change.address);
        let old_code = hex::encode(&code_change.old_code);
        let old_hash = hex::encode(&code_change.old_hash);
        let new_code = hex::encode(&code_change.new_code);
        let new_hash = hex::encode(&code_change.new_hash);
        let ordinal = &code_change.ordinal;

        save_ethereum_block_code_changes(
            id,
            block_number,
            parent_id,
            parent_table,
            address,
            old_code,
            old_hash,
            new_code,
            new_hash,
            ordinal,
            database_changes
        );
    }

    // block system_calls[]
    let system_calls = block.system_calls;
    for (call_index,system_call) in system_calls.iter().enumerate(){
        let id = format!("{}_{}",block_number,call_index);
        let parent_id = format!("{}",block_number);
        let parent_table = "ethereum_block".to_string();
        let index = system_call.index;
        let parent_index = system_call.parent_index;
        let depth = system_call.depth;
        let call_type = system_call.call_type;
        let caller = hex::encode(&system_call.caller);
        let address = hex::encode(&system_call.address);
        let system_call_value = match &system_call.value.clone() {
            Some(val) => BigInt::from_bytes_be(num_bigint::Sign::Plus,&val.bytes).to_string(),None => String::new()
        };
        let gas_limit = system_call.gas_limit;
        let gas_consumed = system_call.gas_consumed;
        let return_data = hex::encode(&system_call.return_data);
        let input = hex::encode(&system_call.input);
        let executed_code = system_call.executed_code;
        let suicide = system_call.suicide;
        let status_failed = system_call.status_failed;
        let status_reverted = system_call.status_reverted;
        let failure_reason: &String = &system_call.failure_reason;
        let state_reverted = system_call.state_reverted;
        let begin_ordinal = system_call.begin_ordinal;
        let end_ordinal = system_call.end_ordinal;

        save_ethereum_block_system_calls(
            id,
            block_number,
            parent_id,
            parent_table,
            call_index,
            index,
            parent_index,
            depth,
            call_type,
            caller,
            address,
            system_call_value,
            gas_limit,
            gas_consumed,
            return_data,
            input,
            executed_code,
            suicide,
            status_failed,
            status_reverted,
            failure_reason,
            state_reverted,
            begin_ordinal,
            end_ordinal,
            database_changes
        );

        // block system_calls storage_change
        let storage_changes = &system_call.storage_changes;
        for (storage_change_index, storage_change) in storage_changes.iter().enumerate(){
            let id = format!("{}_{}_{}",block_number,call_index,storage_change_index);
            let parent_id = format!("{}_{}",block_number,call_index);
            let parent_table = "ethereum_block_system_calls".to_string();
            let address = hex::encode(&storage_change.address);
            let key = hex::encode(&storage_change.key);
            let old_value = hex::encode(&storage_change.old_value);
            let new_value  = hex::encode(&storage_change.new_value);
            let ordinal = storage_change.ordinal;
            save_ethereum_block_system_calls_storage_changes(
                id,
                block_number,
                parent_id,
                parent_table,
                storage_change_index,
                address,
                key,
                old_value,
                new_value,
                ordinal,
                database_changes
            );

        }

        // block system_calls balance_changes 
        let balance_changes = &system_call.balance_changes;
        for (balance_change_index,balance_change) in balance_changes.iter().enumerate() {
            let id = format!("{}_{}_{}",block_number,call_index,balance_change_index);
            let parent_id = format!("{}_{}",block_number,call_index);
            let parent_table = "ethereum_block_system_calls".to_string();
            let address = hex::encode(&balance_change.address);
            let old_value = match &balance_change.old_value.clone() {
                Some(val) => hex::encode(&val.bytes),
                None => String::new()
            };
            let new_value  = match &balance_change.new_value.clone() {
                Some(val) => hex::encode(&val.bytes),
                None => String::new()
            };
            let ordinal = balance_change.ordinal;
            save_ethereum_block_system_calls_balance_changes(
                id,
                block_number,
                parent_id,
                parent_table,
                balance_change_index,
                address,
                old_value,
                new_value,
                ordinal,
                database_changes
            );

        }

        // block system_calls nonce_changes
        let nonce_changes = &system_call.nonce_changes;
        for (nonce_change_index,nonce_change) in nonce_changes.iter().enumerate(){
            let id = format!("{}_{}_{}",block_number,call_index,nonce_change_index);
            let parent_id = format!("{}_{}",block_number,call_index);
            let parent_table = "ethereum_block_system_calls".to_string();
            let address = hex::encode(&nonce_change.address);
            let old_value = nonce_change.old_value;
            let new_value  = nonce_change.new_value;
            let ordinal = nonce_change.ordinal;

            save_ethereum_block_system_calls_nonce_changes(
                id,
                block_number,
                parent_id,
                parent_table,
                nonce_change_index,
                address,
                old_value,
                new_value,
                ordinal,
                database_changes
            );
        }

        // block system_calls code_changes
        let code_changes = &system_call.code_changes;
        for (code_change_index , code_change)  in code_changes.iter().enumerate(){
            let id = format!("{}_{}_{}",block_number,call_index,code_change_index);
            let parent_id = format!("{}_{}",block_number,call_index);
            let parent_table = "ethereum_block_system_calls".to_string();
            let address = hex::encode(&code_change.address);
            let old_code = hex::encode(&code_change.old_code);
            let new_code  = hex::encode(&code_change.new_code);
            let old_hash = hex::encode(&code_change.old_code);
            let new_hash = hex::encode(&code_change.new_hash);
            let ordinal = code_change.ordinal;

            save_ethereum_block_system_calls_code_changes(
                id,
                block_number,
                parent_id,
                parent_table,
                code_change_index,
                address,
                old_code,
                new_code,
                old_hash,
                new_hash,
                ordinal,
                database_changes
            );

        }
        // block system_calls gas_changes
        let gas_changes = &system_call.gas_changes;
        for (gas_change_index , gas_change) in gas_changes.iter().enumerate(){
            let id = format!("{}_{}_{}",block_number,call_index,gas_change_index);
            let parent_id = format!("{}_{}",block_number,call_index);
            let parent_table = "ethereum_block_system_calls".to_string();
            let old_value = gas_change.old_value;
            let new_value  = gas_change.new_value;
            let reason = gas_change.reason;
            let ordinal = gas_change.ordinal;

            save_ethereum_block_system_calls_gas_changes(
                id,
                block_number,
                parent_id,
                parent_table,
                gas_change_index,
                old_value,
                new_value,
                reason,
                ordinal,
                database_changes
            );
        }
        // block system_calls account_creations
        let account_creations = &system_call.account_creations;
        for (account_creation_index , account_creation) in account_creations.iter().enumerate(){
            let id = format!("{}_{}_{}",block_number,call_index,account_creation_index);
            let parent_id = format!("{}_{}",block_number,call_index);
            let parent_table = "ethereum_block_system_calls".to_string();
            let account = hex::encode(&account_creation.account);
            let ordinal  = account_creation.ordinal;

            save_ethereum_block_system_calls_account_creations(
                id,
                block_number,
                parent_id,
                parent_table,
                account_creation_index, 
                account,
                ordinal,
                database_changes
            );
        }


    }




}

fn save_block(block:Block,database_changes: &mut DatabaseChanges){
    
}



fn save_ethereum_block_system_calls_account_creations(
    id: String,
    block_number:u64,
    parent_id:String,
    parent_table:String,
    change_index: usize,
    account : String,
    ordinal : u64,
    changes : &mut DatabaseChanges
){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id);
    changes.push_change_composite("ethereum_block_system_calls_account_creations", keys, 1, Operation::Create)
        .change("block_number", (None,block_number))
        .change("parent_id", (None, parent_id))
        .change("parent_table", (None, parent_table))
        .change("change_index", (None, change_index as u64))
        .change("account", (None,account))
        .change("ordinal", (None,ordinal)) ;
}

fn save_ethereum_block_system_calls_gas_changes(
    id: String,
    block_number:u64,
    parent_id:String,
    parent_table:String,
    change_index: usize,
    old_value:u64,
    new_value:u64,
    reason:i32,
    ordinal:u64,
    changes:&mut DatabaseChanges
){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id);
    changes.push_change_composite("ethereum_block_system_calls_gas_changes", keys, 1, Operation::Create)
        .change("block_number", (None,block_number))
        .change("parent_id", (None, parent_id))
        .change("parent_table", (None, parent_table))
        .change("change_index", (None, change_index as u64)) // usize -> u64
        .change("old_value", (None, old_value))
        .change("new_value", (None, new_value))
        .change("reason", (None, reason))
        .change("ordinal", (None, ordinal));
}

fn save_ethereum_block_system_calls_code_changes(
    id: String,
    block_number:u64,
    parent_id:String,
    parent_table:String,
    code_change_index: usize,
    address:String,
    old_code:String,
    new_code:String,
    old_hash:String,
    new_hash:String,
    ordinal : u64,
    changes : &mut DatabaseChanges
){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id);
    changes.push_change_composite("ethereum_block_system_calls_code_changes", keys, 1, Operation::Create)
        .change("block_number", (None,block_number))
        .change("parent_id", (None, parent_id))
        .change("parent_table", (None, parent_table))
        .change("change_index", (None, code_change_index as u64)) // usize -> u64
        .change("address", (None, address))
        .change("old_code", (None, old_code))
        .change("new_code", (None, new_code))
        .change("old_hash", (None, old_hash))
        .change("new_hash", (None, new_hash))
        .change("ordinal", (None, ordinal));
}


fn save_ethereum_block_system_calls_nonce_changes(
    id: String,
    block_number:u64,
    parent_id:String,
    parent_table:String,
    balance_change_index: usize,
    address : String,
    old_value : u64,
    new_value : u64,
    ordinal : u64,
    changes : &mut DatabaseChanges
){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id);
    changes.push_change_composite("ethereum_block_system_calls_nonce_changes", keys, 1, Operation::Create)
        .change("block_number", (None,block_number))
        .change("parent_id", (None, parent_id))
        .change("parent_table", (None, parent_table))
        .change("change_index", (None, balance_change_index as u64)) // usize -> u64
        .change("address", (None, address))
        .change("old_value", (None, old_value))
        .change("new_value", (None, new_value))
        .change("ordinal", (None, ordinal));
}

fn save_ethereum_block_system_calls_balance_changes(
    id: String,
    block_number:u64,
    parent_id:String,
    parent_table:String,
    balance_change_index: usize,
    address : String,
    old_value : String,
    new_value : String,
    ordinal : u64,
    changes : &mut DatabaseChanges
){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id);
    changes.push_change_composite("ethereum_block_system_calls_balance_changes", keys, 1, Operation::Create)
        .change("block_number", (None,block_number))
        .change("parent_id", (None, parent_id))
        .change("parent_table", (None, parent_table))
        .change("change_index", (None, balance_change_index as u64)) // usize -> u64
        .change("address", (None, address))
        .change("old_value", (None, old_value))
        .change("new_value", (None, new_value))
        .change("ordinal", (None, ordinal));
}

fn save_ethereum_block_system_calls_storage_changes(
    id : String,
    block_number : u64,
    parent_id : String,
    parent_table : String,
    storage_change_index : usize,
    address : String,
    key : String,
    old_value :String,
    new_value : String,
    ordinal : u64,
    changes : &mut DatabaseChanges
){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id);
    changes.push_change_composite("ethereum_block_system_calls_storage_changes", keys, 1, Operation::Create)
        .change("block_number", (None,block_number))
        .change("parent_id", (None, parent_id))
        .change("parent_table", (None, parent_table))
        .change("change_index", (None, storage_change_index as u64)) // usize -> u64
        .change("address", (None, address))
        .change("key", (None, key))
        .change("old_value", (None, old_value))
        .change("new_value", (None, new_value))
        .change("ordinal", (None, ordinal));
}

fn save_ethereum_block_system_calls(
    id :String,
    block_number: u64,
    parent_id: String,
    parent_table : String,
    call_index : usize,
    index : u32,
    parent_index:u32,
    depth:u32,
    call_type:i32,
    caller:String,
    address:String,
    system_call_value:String,
    gas_limit : u64,
    gas_consumed:u64,
    return_data:String,
    input:String,
    executed_code:bool,
    suicide:bool,
    status_failed:bool,
    status_reverted:bool,
    failure_reason:&String,
    state_reverted:bool,
    begin_ordinal:u64,
    end_ordinal:u64,
    changes:&mut DatabaseChanges
){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id);
    changes.push_change_composite("ethereum_block_system_calls", keys, 1, Operation::Create)
        .change("block_number", (None,block_number))
        .change("parent_id", (None, parent_id))
        .change("parent_table", (None, parent_table))
        .change("system_call_index", (None, call_index as u64)) // usize -> u64
        .change("index", (None, index))
        .change("parent_index", (None, parent_index))
        .change("depth", (None, depth))
        .change("call_type", (None, call_type))
        .change("caller", (None, caller))
        .change("address", (None, address))
        .change("system_call_value", (None, system_call_value))
        .change("gas_limit", (None, gas_limit))
        .change("gas_consumed", (None, gas_consumed))
        .change("return_data", (None, return_data))
        .change("input", (None, input))
        .change("executed_code", (None, executed_code))
        .change("suicide", (None, suicide))
        .change("status_failed", (None, status_failed))
        .change("status_reverted", (None, status_reverted))
        .change("failure_reason", (None, failure_reason.clone())) // 解引用并克隆
        .change("state_reverted", (None, state_reverted))
        .change("begin_ordinal", (None, begin_ordinal))
        .change("end_ordinal", (None, end_ordinal));
}




fn save_ethereum_block_code_changes(
    id : String,
    block_number : u64,
    parent_id : String,
    parent_table : String,
    address : String,
    old_code : String,
    old_hash : String,
    new_code : String,
    new_hash : String,
    ordinal : &u64,
    changes : &mut DatabaseChanges
){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id);
    changes.push_change_composite("ethereum_block_code_changes", keys, 1, Operation::Create)
        .change("block_number", (None,block_number))
        .change("parent_id", (None, parent_id))
        .change("parent_table", (None, parent_table))
        .change("address", (None,address))
        .change("old_hash", (None,old_hash ))
        .change("old_code", (None,old_code))
        .change("new_code", (None,new_code))
        .change("new_hash", (None,new_hash))
        .change("ordinal", (None,*ordinal));
}

fn save_ethereum_block_balance_changes(
    id : String,
    block_number:u64,
    parent_id : String,
    parent_table : String,
    address : String,
    old_value : String,
    new_value : String,
    reason : &i32,
    ordinal : &u64,
    changes : &mut DatabaseChanges
){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id);
    changes.push_change_composite("ethereum_block_balance_changes", keys, 1, Operation::Create)
        .change("block_number", (None,block_number))
        .change("parent_id", (None, parent_id))
        .change("parent_table", (None, parent_table))
        .change("address", (None,address))
        .change("old_value", (None,old_value ))
        .change("new_value", (None,new_value))
        .change("reason", (None,*reason))
        .change("ordinal", (None,*ordinal));
}



fn save_ethereum_block_transaction_receipt_logs_topics(
    id : String,
    block_number : u64,
    parent_id : String,
    parent_table:String,
    log_index:usize,
    topic_index:usize,
    topic : String,
    changes : &mut DatabaseChanges
){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id);
    changes.push_change_composite("ethereum_block_transaction_receipt_logs_topics", keys, 1, Operation::Create)
        .change("block_number", (None,block_number))
        .change("parent_id", (None, parent_id))
        .change("parent_table", (None, parent_table))
        .change("log_index", (None,log_index as u64))
        .change("topic_index", (None,topic_index as u64))
        .change("topic", (None,topic));
}

fn save_ethereum_block_transaction_receipt_logs(
    id:String,
    block_number:u64,
    parent_id: String,
    parent_table : String,
    log_index:usize,
    address:String,
    data : String,
    index : &u32,
    block_index : &u32,
    ordinal : &u64,
    changes : &mut DatabaseChanges
){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id);
    changes.push_change_composite("ethereum_block_transaction_receipt_logs", keys, 1, Operation::Create)
        .change("block_number", (None,block_number))
        .change("parent_id", (None, parent_id))
        .change("parent_table", (None, parent_table))
        .change("log_index", (None,log_index as u64))
        .change("address", (None,address))
        .change("data", (None,data))
        .change("index", (None, *index)) 
        .change("block_index", (None, *block_index))
        .change("ordinal", (None, *ordinal));
}


fn save_ethereum_block_transaction_receipt(
    id : String,
    block_number : u64,
    parent_id : String,
    parent_table : String,
    transaction_index : usize,
    state_root : String,
    cumulative_gas_used : u64,
    logs_bloom : String,
    blob_gas_used : u64,
    blob_gas_price : String,
    changes : &mut DatabaseChanges
){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert("id".to_string(), id);
    changes.push_change_composite("ethereum_block_transaction_receipt", keys, 1, Operation::Create)
        .change("block_number", (None,block_number))
        .change("parent_id", (None, parent_id))
        .change("parent_table", (None, parent_table))
        .change("transaction_index", (None,transaction_index as u64))
        .change("state_root", (None,state_root))
        .change("cumulative_gas_used", (None,cumulative_gas_used))
        .change("logs_bloom", (None,logs_bloom))
        .change("blob_gas_used", (None,blob_gas_used))
        .change("blob_gas_price", (None,blob_gas_price));

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
    changes.push_change_composite("ethereum_block_transaction_access_list_storage_keys", keys, 1, Operation::Create)
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
    changes.push_change_composite("ethereum_block_transaction_access_list", keys, 1, Operation::Create)
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
    changes.push_change_composite("ethereum_block_transaction", keys, 1, Operation::Create)
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
    
}