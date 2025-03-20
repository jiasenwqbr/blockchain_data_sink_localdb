use std::collections::HashMap;

use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};

use crate::pb::sf::solana::r#type::v1::Block;

// Vote111111111111111111111111111111111111111
static VOTE_INSTRUCTION: &'static [u8] = &[
    7, 97, 72, 29, 53, 116, 116, 187, 124, 77, 118, 36, 235, 211, 189, 179, 216, 53, 94, 115, 209, 16, 67, 252, 13,
    163, 83, 128, 0, 0, 0, 0,
];

#[substreams::handlers::map]
fn out_db(mut block: Block) -> Result<DatabaseChanges, substreams::errors::Error> {
    block.transactions.retain(|trx| {
        let meta = match trx.meta.as_ref() {
            Some(meta) => meta,
            None => return false,
        };
        if meta.err.is_some() {
            return false;
        }

        let transaction = match trx.transaction.as_ref() {
            Some(transaction) => transaction,
            None => return false,
        };
        let message = transaction.message.as_ref().expect("Message is missing");

        if message.account_keys.contains(&VOTE_INSTRUCTION.to_vec()) {
            return false;
        }

        true
    });

    let mut database_changes: DatabaseChanges = Default::default();
    let json = serde_json::to_string_pretty(&block).expect("序列化失败");
    let block_number = block.block_height.unwrap_or_default().block_height;
    let mut composite_key: HashMap<String, String> = HashMap::new();
    composite_key.insert("id".to_string(), block_number.to_string());
    database_changes
        .push_change_composite("solana_block", composite_key, 1, Operation::Create)
        .change("data", (None, json));

    Ok(database_changes)
}
