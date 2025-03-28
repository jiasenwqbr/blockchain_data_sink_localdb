use std::collections::HashMap;

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

    // block header
    
}
