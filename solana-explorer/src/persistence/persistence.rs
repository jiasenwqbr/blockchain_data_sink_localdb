use std::collections::HashMap;

use crate::pb::sf::solana::r#type::v1::Block;
use substreams_database_change::{pb::database::{table_change::Operation, DatabaseChanges}};
use substreams_solana::base58;
pub fn save_solana_block_all(block: Block, database_changes: &mut DatabaseChanges) {
    let json = serde_json::to_string_pretty(&block).expect("序列化失败");
    let block_number = block.block_height.unwrap_or_default().block_height;
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert("id".to_string(), block_number.to_string());
    database_changes
        .push_change_composite("solana_block", composite_key, 1, Operation::Create)
        .change("data", (None, json));
    let block_hash = block.blockhash;
    let previous_blockhash = block.previous_blockhash;
    let parent_slot = block.parent_slot;
    let block_height = block.block_height.unwrap_or_default().block_height;
    let block_number = block.slot;
    let block_time: i64 = block.block_time.unwrap_or_default().timestamp;
    // head
    save_solana_block_head(
        block_number,
        block_hash,
        previous_blockhash,
        parent_slot,
        block_height,
        block_time,
        database_changes,
    );

    let transactions: Vec<crate::pb::sf::solana::r#type::v1::ConfirmedTransaction> = block.transactions;
    for (index, transaction) in transactions.iter().enumerate() {
        match &transaction.transaction {
            Some(tx) => {
                let singnatures = &tx.signatures;
                for (ind, signature) in singnatures.iter().enumerate() {
                    let sig = base58::encode(signature);
                    save_solana_block_transaction_signature(block_number, sig, index, ind, database_changes);
                }
                // message
                match &tx.message {
                    Some(msg) => {
                        let num_required_signatures = msg.header.unwrap().num_required_signatures;
                        let num_readonly_signed_accounts = msg.header.unwrap_or_default().num_readonly_signed_accounts;
                        let num_readonly_unsigned_accounts =
                            msg.header.unwrap_or_default().num_readonly_unsigned_accounts;
                        let versioned = msg.versioned;
                        let recent_blockhash = base58::encode(&msg.recent_blockhash);
                        save_solana_block_transaction_message(
                            block_number,
                            index,
                            num_required_signatures,
                            num_readonly_signed_accounts,
                            num_readonly_unsigned_accounts,
                            recent_blockhash,
                            versioned,
                            database_changes
                        );
                        // block transaction message Instruction 
                        let instructions = &msg.instructions;
                        for (ins_index,instruction) in instructions.iter().enumerate(){
                            let accounts = base58::encode(&instruction.accounts);
                            let instruction_data = base58::encode(&instruction.data);
                            let program_id_index = instruction.program_id_index;

                            save_solana_block_transaction_message_instruction(
                                block_number,
                                index,
                                program_id_index,
                                accounts,
                                instruction_data,
                                ins_index,
                                database_changes
                            );
                        }
                        // block transaction message account_keys
                        let account_keys: &Vec<Vec<u8>> = &msg.account_keys;
                        for (account_indx,ak) in account_keys.iter().enumerate(){
                            let account_key = base58::encode(ak);
                            save_solana_block_transaction_message_account_key(block_number,index,account_key,account_indx,database_changes);
                        }

                        // block transaction message address_table_lookups
                        let address_table_lookups: &Vec<crate::pb::sf::solana::r#type::v1::MessageAddressTableLookup>  = &msg.address_table_lookups;
                        for (atl_index,atl) in address_table_lookups.iter().enumerate() {
                            let account_key  = base58::encode(&atl.account_key);
                            let writable_indexes = base58::encode(&atl.writable_indexes);
                            let readonly_indexes = base58::encode(&atl.readonly_indexes);
                            save_solana_block_transaction_message_address_table_lookups(
                                block_number,index,atl_index,account_key,writable_indexes,readonly_indexes,database_changes
                            );

                        }
                    }
                    None => {}
                }
            }
            None => {}
        }
        match  &transaction.meta {
            Some(meta) =>{
                // block TransactionStatusMeta
                let fee = meta.fee;
                let inner_instructions_none = meta.inner_instructions_none;
                let log_messages_none: bool = meta.log_messages_none;
                let return_data_none = meta.return_data_none;
                let return_data_program_id = base58::encode(meta.return_data.clone().unwrap_or_default().program_id);
                let return_data_data = base58::encode(meta.return_data.clone().unwrap_or_default().data);
                let parent_table_id = format!("{}",block_number);
                save_solana_block_transaction_status_meta(block_number,index,fee,inner_instructions_none,log_messages_none,return_data_none,return_data_program_id,return_data_data,parent_table_id,database_changes);

            },
            None => {}
        }

    }
    
    
}
fn save_solana_block_transaction_status_meta(
    block_number:u64,
    transaction_index:usize,
    fee:u64,
    inner_instructions_none:bool,
    log_messages_none:bool,
    return_data_none:bool,
    return_data_program_id:String,
    return_data_data:String,
    parent_table_id:String,
    changes:&mut DatabaseChanges){
        let mut composite_key: HashMap<String, String> = HashMap::new();
        composite_key.insert("id".to_string(), format!("{}_{}",block_number,transaction_index));
        changes
            .push_change_composite("solana_block_transaction_status_meta", composite_key, 1, Operation::Create)
            .change("block_number", (None,block_number))
            .change("transaction_index", (None,transaction_index as u64))
            .change("fee", (None,fee))
            .change("inner_instructions_none", (None,inner_instructions_none))
            .change("log_messages_none", (None,log_messages_none))
            .change("return_data_none", (None,return_data_none))
            .change("return_data_program_id", (None,return_data_program_id))
            .change("return_data_data", (None,return_data_data))
            .change("parent_table_id", (None,parent_table_id));
    }


fn save_solana_block_head(
    block_number: u64,
    block_hash: String,
    previous_blockhash: String,
    parent_slot: u64,
    block_height: u64,
    block_time: i64,
    changes: &mut DatabaseChanges,
) {
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert("id".to_string(), block_number.to_string());
    changes
        .push_change_composite("solana_block_head", composite_key, 1, Operation::Create)
        .change("previous_blockhash", (None, previous_blockhash))
        .change("block_hash", (None, block_hash))
        .change("parent_slot", (None, parent_slot))
        .change("block_height", (None, block_height))
        .change("block_time", (None, block_time))
        .change("block_number", (None, block_number));
}

fn save_solana_block_transaction_signature(
    block_number: u64,
    signature: String,
    transaction_index: usize,
    signature_index: usize,
    changes: &mut DatabaseChanges,
) {
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert(
        "id".to_string(),
        format!("{}_{}_{}", block_number, transaction_index, signature_index),
    );
    changes
        .push_change_composite(
            "solana_block_transaction_signature",
            composite_key,
            1,
            Operation::Create,
        )
        .change("block_number", (None, block_number))
        .change("signature", (None, signature))
        .change("transaction_index", (None, transaction_index as u64))
        .change("signature_index", (None, signature_index as u64));
}

fn save_solana_block_transaction_message(
    block_number: u64,
    transaction_index: usize,
    num_required_signatures: u32,
    num_readonly_signed_accounts: u32,
    num_readonly_unsigned_accounts: u32,
    recent_blockhash: String,
    versioned: bool,
    changes: &mut DatabaseChanges,
) {
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert(
        "id".to_string(),
        format!("{}_{}", block_number, transaction_index),
    );
    changes
        .push_change_composite(
            "solana_block_transaction_message",
            composite_key,
            1,
            Operation::Create,
        )
        .change("block_number", (None,block_number))
        .change("transaction_index", (None,transaction_index as u64))
        .change("header_num_required_signatures", (None,num_required_signatures))
        .change("header_num_readonly_signed_accounts", (None,num_readonly_signed_accounts))
        .change("header_num_readonly_unsigned_accounts", (None,num_readonly_unsigned_accounts))
        .change("recent_blockhash", (None,recent_blockhash))
        .change("versioned", (None,versioned));
}


fn save_solana_block_transaction_message_instruction(
    block_number:u64,
    transaction_index : usize,
    program_id_index:u32,
    instruction_accounts:String,
    instruction_data:String,
    instruction_index:usize,
    changes: &mut DatabaseChanges,
){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert(
        "id".to_string(),
        format!("{}_{}_{}", block_number, transaction_index,instruction_index),
    );
    changes.push_change_composite("solana_block_transaction_message_instruction", keys, 1, Operation::Create)
    .change("block_number", (None,block_number))
    .change("transaction_index", (None,transaction_index as u64))
    .change("program_id_index", (None,program_id_index))
    .change("instruction_accounts", (None,instruction_accounts))
    .change("instruction_data", (None,instruction_data))
    .change("instruction_index", (None,instruction_index as u64));
}

fn save_solana_block_transaction_message_account_key(block_number:u64,transaction_index:usize,account_key:String,account_indx:usize,changes: &mut DatabaseChanges,){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert(
        "id".to_string(),
        format!("{}_{}_{}", block_number, transaction_index,account_indx),
    );
    changes.push_change_composite("solana_block_transaction_message_account_key", keys, 1, Operation::Create)
    .change("block_number", (None,block_number))
    .change("transaction_index", (None,transaction_index as u64))
    .change("account_key", (None,account_key))
    .change("account_key_index", (None,account_indx as u64));
}

fn save_solana_block_transaction_message_address_table_lookups(
    block_number:u64,
    transaction_index:usize,
    address_table_lookups_index:usize,
    account_key:String,
    writable_indexes:String,
    readonly_indexes:String,
    changes:&mut DatabaseChanges
){
    let mut keys: HashMap<String, String> = HashMap::new();
    keys.insert(
        "id".to_string(),
        format!("{}_{}_{}", block_number, transaction_index,address_table_lookups_index),
    );
    changes.push_change_composite("solana_block_transaction_message_address_table_lookups", keys, 1, Operation::Create)
    .change("block_number", (None,block_number))
    .change("transaction_index", (None,transaction_index as u64))
    .change("account_key", (None,account_key))
    .change("writable_indexes", (None,writable_indexes))
    .change("readonly_indexes", (None,readonly_indexes))
    .change("address_table_lookups_index", (None,address_table_lookups_index as u64));
}

