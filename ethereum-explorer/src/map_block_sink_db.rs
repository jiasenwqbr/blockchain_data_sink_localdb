use std::collections::HashMap;

use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};

use crate::pb::eth::ethereum::v2::Block;



#[substreams::handlers::map]
fn db_out(block: Block) -> Result<DatabaseChanges, substreams::errors::Error> {
    let data = serde_json::to_string_pretty(&block).expect("序列化失败");
    
    let mut database_changes: DatabaseChanges = Default::default();
    let block_number = block.number;
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert("id".to_string(), block_number.to_string());
    database_changes
        .push_change_composite("eth_block", composite_key, 1, Operation::Create)
        .change("data", (None, data));

    Ok(database_changes)
}