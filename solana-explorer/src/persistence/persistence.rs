use std::{collections::HashMap, usize};

// use crate::pb::sf::solana::r#type::v1::Block;
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use substreams_solana::{base58, pb::sf::solana::r#type::v1::Block};
pub fn save_solana_block_all(block: Block, database_changes: &mut DatabaseChanges) {
    // let json = serde_json::to_string_pretty(&block).expect("序列化失败");
    // let block_number = block.block_height.unwrap_or_default().block_height;
    // let mut composite_key: HashMap<String, String> = HashMap::new();
    // composite_key.insert("id".to_string(), block_number.to_string());
    // database_changes
    //     .push_change_composite("solana_block", composite_key, 1, Operation::Create)
    //     .change("data", (None, json));


    let block_hash = block.blockhash;
    let previous_blockhash = block.previous_blockhash;
    let parent_slot = block.parent_slot;
    let block_height = match  block.block_height {
        Some(val) => val.block_height,
        None => 0
    };
    let block_number = block.slot;
    let block_time: i64 = match block.block_time {
        Some(val) => val.timestamp,
        None => 0
    };
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

    let transactions= block.transactions;
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
                        //let num_required_signatures = msg.header.unwrap().num_required_signatures;
                        let num_required_signatures =  match &msg.header {
                            Some(val) => val.num_required_signatures,
                            None => 0
                        };
                        // let num_readonly_signed_accounts = msg.header.unwrap_or_default().num_readonly_signed_accounts;
                        let num_readonly_signed_accounts = match &msg.header {
                            Some(val) => val.num_readonly_signed_accounts,
                            None => 0
                        };
                        // let num_readonly_unsigned_accounts =msg.header.unwrap_or_default().num_readonly_unsigned_accounts;
                        let num_readonly_unsigned_accounts = match &msg.header {
                            Some(val) => val.num_readonly_unsigned_accounts,
                            None => 0
                        };
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
                        let address_table_lookups = &msg.address_table_lookups;
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
                // let return_data_program_id = base58::encode(meta.return_data.clone().unwrap_or_default().program_id);
                let return_data_program_id = match  &meta.return_data {
                    Some(val) => base58::encode(&val.program_id),
                    None => String::new()
                    
                };
                // let return_data_data = base58::encode(meta.return_data.clone().unwrap_or_default().data);

                let return_data_data = match &meta.return_data {
                    Some(val) => base58::encode(&val.data),
                    None => String::new()
                    
                };
                let parent_table_id = format!("{}",block_number);
                save_solana_block_transaction_status_meta(block_number,index,fee,inner_instructions_none,log_messages_none,return_data_none,return_data_program_id,return_data_data,parent_table_id,database_changes);
                // block TransactionStatusMeta  InnerInstructions
                let inner_instructions= &meta.inner_instructions;
                for (inner_instructions_index,ii) in inner_instructions.iter().enumerate(){
                    let ii_index = ii.index;
                    for (inner_instruction_index ,instruction) in ii.instructions.iter().enumerate(){
                        let program_id_index = instruction.program_id_index;
                        let accounts = base58::encode(&instruction.accounts);
                        let instruction_data: String = base58::encode(&instruction.data);
                        // let data = instruction.data;
                        let stack_height = instruction.stack_height.unwrap();
                        let parent_table_id = format!("{}_{}",block_number,index);
                        save_solana_block_transaction_status_meta_inner_instruction(
                            block_number,
                            index,
                            inner_instructions_index,
                            inner_instruction_index,
                            ii_index,
                            program_id_index,
                            accounts,
                            instruction_data,
                            stack_height,
                            parent_table_id,
                            database_changes
                        );
                    }
                }
                // block TransactionStatusMeta  token_balances 
                let pre_token_balances = &meta.pre_token_balances;
                for (pre_token_balance_index, token_balance) in pre_token_balances.iter().enumerate() {
                    let account_index  = token_balance.account_index;
                    let mint = token_balance.mint.clone();
                    let owner = token_balance.owner.clone();
                    let program_id = token_balance.program_id.clone();
                    // let ui_token_amount_ui_amount = token_balance.ui_token_amount.clone().unwrap().ui_amount;
                    let ui_token_amount_ui_amount = match  &token_balance.ui_token_amount {
                        Some(val) => val.ui_amount,
                        None => 0.0
                    };
                    // let ui_token_amount_decimals = token_balance.ui_token_amount.clone().unwrap().decimals;
                    let ui_token_amount_decimals = match  &token_balance.ui_token_amount{
                        Some(val) => val.decimals,
                        None => 0
                    };
                    // let ui_token_amount_amount = token_balance.ui_token_amount.clone().unwrap().amount;
                    let ui_token_amount_amount = match &token_balance.ui_token_amount {
                        Some(val) => val.amount.clone(),
                        None => String::new()
                    };
                    // let ui_token_amount_ui_amount_string = token_balance.ui_token_amount.clone().unwrap().ui_amount_string;
                    let ui_token_amount_ui_amount_string = match &token_balance.ui_token_amount {
                        Some(val) => val.ui_amount_string.clone(),
                        None => String::new()
                    };
                    let flag = 0;
                    let parent_table_id = format!("{}_{}",block_number,index);
                    save_solana_block_transaction_status_meta_pre_post_token_balances(
                        block_number,
                        index,
                        flag,
                        pre_token_balance_index,
                        account_index,
                        mint,
                        ui_token_amount_ui_amount,
                        ui_token_amount_decimals,
                        ui_token_amount_amount,
                        ui_token_amount_ui_amount_string,
                        owner,
                        program_id,
                        parent_table_id,
                        database_changes

                    );

                }
                let post_token_balances= &meta.post_token_balances;
                for (post_token_balance_index, token_balance) in post_token_balances.iter().enumerate() {
                    let account_index  = token_balance.account_index;
                    let mint = token_balance.mint.clone();
                    let owner = token_balance.owner.clone();
                    let program_id = token_balance.program_id.clone();
                    //let ui_token_amount_ui_amount = token_balance.ui_token_amount.clone().unwrap().ui_amount;
                    let ui_token_amount_ui_amount = match &token_balance.ui_token_amount {
                        Some(val) => val.ui_amount,
                        None => 0.0
                    };
                    // let ui_token_amount_decimals = token_balance.ui_token_amount.clone().unwrap().decimals;
                    let ui_token_amount_decimals = match  &token_balance.ui_token_amount {
                        Some(val) => val.decimals,
                        None  => 0
                    };
                    //let ui_token_amount_amount = token_balance.ui_token_amount.clone().unwrap().amount;
                    let ui_token_amount_amount = match &token_balance.ui_token_amount {
                        Some(val) => val.amount.clone(),
                        None => String::new()
                    };
                    //let ui_token_amount_ui_amount_string = token_balance.ui_token_amount.clone().unwrap().ui_amount_string;
                    let ui_token_amount_ui_amount_string = match  &token_balance.ui_token_amount {
                        Some(val) => val.ui_amount_string.clone(),
                        None => String::new()
                    };
                    let flag = 1;
                    let parent_table_id: String = format!("{}_{}",block_number,index);
                    save_solana_block_transaction_status_meta_pre_post_token_balances(
                        block_number,
                        index,
                        flag,
                        post_token_balance_index,
                        account_index,
                        mint,
                        ui_token_amount_ui_amount,
                        ui_token_amount_decimals,
                        ui_token_amount_amount,
                        ui_token_amount_ui_amount_string,
                        owner,
                        program_id,
                        parent_table_id,
                        database_changes

                    );
                }
            
                // block TransactionStatusMeta  rewards
                let rewards = &meta.rewards;
                for (rewoard_index,reward) in rewards.iter().enumerate(){
                    let pubkey = reward.pubkey.clone();
                    let lamports = reward.lamports;
                    let post_balance = reward.post_balance;
                    let reward_type = reward.reward_type;
                    let commission = reward.commission.clone();
                    let parent_table_id = format!("{}_{}",block_number,index);
                    save_solana_block_transaction_status_meta_rewards(
                        block_number,
                        index,
                        rewoard_index,
                        pubkey,
                        lamports,
                        post_balance,
                        reward_type,
                        commission,
                        parent_table_id,
                        database_changes
                    );
                }
            },
            None => {}
        }
    }

    let rewards = block.rewards;
    for (reward_index,reward) in rewards.iter().enumerate(){
        let pubkey = reward.pubkey.clone();
        let lamports = reward.lamports;
        let post_balance = reward.post_balance;
        let reward_type = reward.reward_type;
        let commission = reward.commission.clone();
        let parent_table_id = format!("{}",block_number);
        save_solana_block_rewards(
            block_number,
            reward_index,
            pubkey,
            lamports,
            post_balance,
            reward_type,
            commission,
            parent_table_id,
            database_changes
        );
    }
    
    
}

fn save_solana_block_rewards(
    block_number : u64,
    reward_index : usize,
    pubkey : String,
    lamports : i64,
    post_balance : u64,
    reward_type : i32,
    commission : String,
    parent_table_id : String,
    changes : &mut DatabaseChanges
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert("id".to_string(), format!("{}_{}",block_number,reward_index));
    changes.push_change_composite("solana_block_rewards", composite_key, 1, Operation::Create)
    .change("block_number", (None,block_number))
    .change("reward_index", (None,reward_index as u64))
    .change("pubkey", (None,pubkey))
    .change("lamports", (None,lamports))
    .change("post_balance", (None,post_balance))
    .change("reward_type", (None,reward_type))
    .change("commission", (None,commission))
    .change("parent_table_id", (None,parent_table_id));
}

fn save_solana_block_transaction_status_meta_rewards(
    block_number:u64,
    transaction_index : usize,
    rewoard_index:usize,
    pubkey : String,
    lamports : i64,
    post_balance : u64,
    reward_type : i32,
    commission : String,
    parent_table_id : String,
    changes: &mut DatabaseChanges
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert("id".to_string(), format!("{}_{}_{}",block_number,transaction_index,rewoard_index));
    changes.push_change_composite("solana_block_transaction_status_meta_pre_post_token_balances", composite_key, 1, Operation::Create)
    .change("block_number", (None,block_number))
    .change("transaction_index", (None,transaction_index as u64))
    .change("reward_index", (None,rewoard_index as u64))
    .change("pubkey", (None,pubkey))
    .change("lamports", (None,lamports))
    .change("post_balance", (None,post_balance))
    .change("reward_type", (None,reward_type))
    .change("commission", (None,commission))
    .change("parent_table_id", (None,parent_table_id));
}

fn save_solana_block_transaction_status_meta_pre_post_token_balances(
    block_number:u64,
    transaction_index:usize,
    pre_post_flag : i32,
    balance_index:usize,
    account_index : u32,
    mint:String,
    ui_token_amount_ui_amount:f64,
    ui_token_amount_decimals:u32,
    ui_token_amount_amount:String,
    ui_token_amount_ui_amount_string:String,
    owner:String,
    program_id:String,
    parent_table_id:String,
    changes:&mut DatabaseChanges

){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert("id".to_string(), format!("{}_{}_{}_{}",block_number,transaction_index,balance_index,pre_post_flag));
    changes.push_change_composite("solana_block_transaction_status_meta_pre_post_token_balances", composite_key, 1, Operation::Create)
    .change("block_number", (None,block_number))
    .change("transaction_index", (None,transaction_index as u64))
    .change("pre_post_flag", (None,pre_post_flag))
    .change("balance_index", (None,balance_index as u64))
    .change("account_index", (None,account_index))
    .change("mint", (None,mint))
    .change("ui_token_amount_ui_amount", (None,ui_token_amount_ui_amount.to_string()))
    .change("ui_token_amount_decimals", (None,ui_token_amount_decimals))
    .change("ui_token_amount_amount", (None,ui_token_amount_amount))
    .change("ui_token_amount_ui_amount_string", (None,ui_token_amount_ui_amount_string))
    .change("owner", (None,owner))
    .change("program_id", (None,program_id))
    .change("parent_table_id", (None,parent_table_id));

}
fn save_solana_block_transaction_status_meta_inner_instruction(
    block_number : u64,
    transaction_index:usize,
    inner_instructions_index : usize,
    inner_instruction_index:usize,
    instruction_index : u32,
    program_id_index : u32,
    accounts : String,
    instruction_data : String,
    stack_height : u32,
    parent_table_id : String,
    changes : &mut DatabaseChanges
){
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert("id".to_string(), format!("{}_{}_{}_{}",block_number,transaction_index,inner_instructions_index,inner_instruction_index));
    changes.push_change_composite("solana_block_transaction_status_meta_inner_instruction", composite_key, 1, Operation::Create)
    .change("block_number", (None,block_number))
    .change("transaction_index", (None,transaction_index as u64))
    .change("inner_instructions_index", (None,inner_instructions_index as u64))
    .change("inner_instruction_index", (None,inner_instruction_index as u64))
    .change("instruction_index", (None,instruction_index ))
    .change("program_id_index", (None,program_id_index))
    .change("accounts", (None,accounts))
    .change("instruction_data", (None,instruction_data))
    .change("stack_height", (None,stack_height))
    .change("parent_table_id", (None,parent_table_id));
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

