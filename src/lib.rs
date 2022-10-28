use substreams::store::{StoreAdd, StoreAddInt64, StoreNew};
use substreams_ethereum::pb::eth::v2 as eth;

substreams_ethereum::init!();

#[substreams::handlers::store]
fn store_address_created(blk: eth::Block, s: StoreAddInt64) {
    let account_created = blk.calls().fold(0 as i64, |sum, call| {
        sum + call.call.account_creations.len() as i64
    });

    s.add(blk.number, "total", account_created);
}
