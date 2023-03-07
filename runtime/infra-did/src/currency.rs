#![allow(non_upper_case_globals)]
use common_primitives::types::Balance;

pub const IDID: Balance = 1_000_000_000_000_000_000; // 18 decimal
pub const cIDID: Balance = IDID / 100; // 16 decimal, cent-IDID
pub const mIDID: Balance = IDID / 1_000; //15 decimal, milli-IDID
pub const uIDID: Balance = IDID / 1_000_000; // 12 decimal, micro-IDID

pub const fn deposit(items: u32, bytes: u32) -> Balance {
    items as Balance * 15 * mIDID + (bytes as Balance) * 6 * mIDID // TODO: revisit the storage cost here
}
