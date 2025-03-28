use substreams_database_change::pb::database::DatabaseChanges;
use substreams_ethereum::pb::eth::v2::Block;

use crate::persistence;



#[substreams::handlers::map]
fn postgres_out(block: Block) -> Result<DatabaseChanges, substreams::errors::Error> {

    let mut database_changes: DatabaseChanges = Default::default();
    persistence::persistence::save_ethereum_block(block, &mut database_changes);

    Ok(database_changes)

}