//! Protocol Constants

use crate::types::Balance;
use frame_support::{parameter_types, weights::RuntimeDbWeight, PalletId};

/// InfraDID SS58 Prefix
pub const INFRADID_SS58PREFIX: u8 = 42;

/// InfraDID Decimals
pub const INFRADID_DECIMAL: u8 = 18;

/// InfraDID Token Symbol
pub const INFRADID_TOKEN_SYMBOL: &str = "IDID";

/// parachain time-related
pub mod time {
    use crate::types::{BlockNumber, Moment};

    /// Seconds per Block
    ///
    /// This constant is currently set to 12 seconds.
    ///
    /// This determines the average expected block time that we are targeting. Blocks will be
    /// produced at a minimum duration defined by [`SLOT_DURATION`]. [`SLOT_DURATION`] is picked up
    /// by [`pallet_timestamp`] which is in turn picked up by [`pallet_aura`] to implement the
    /// `slot_duration` function.
    ///
    /// Change this to adjust the block time.
    pub const SECONDS_PER_BLOCK: Moment = 12;

    /// Milliseconds per Block
    pub const MILLISECS_PER_BLOCK: Moment = SECONDS_PER_BLOCK * 1000;

    /// Slot Duration
    pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

    /// Number of Blocks per Minute
    pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);

    /// Number of Blocks per Hour
    pub const HOURS: BlockNumber = MINUTES * 60;

    /// Number of Blocks per Day
    pub const DAYS: BlockNumber = HOURS * 24;
}

/// Asset String Limit
pub const ASSET_STRING_LIMIT: u32 = 50;

/// Staking Pallet Identifier
pub const STAKING_PALLET_ID: PalletId = PalletId(*b"PotStake");

/// Treasury Pallet Identifier
pub const TREASURY_PALLET_ID: PalletId = PalletId(*b"py/trsry");

/// Asset Manager Pallet Identifier
pub const ASSET_MANAGER_PALLET_ID: PalletId = PalletId(*b"asstmngr");

/// Test Default Asset Existential Deposit
///
/// # Warning
///
/// This should only be used for testing and should not be used in production.
pub const TEST_DEFAULT_ASSET_ED: Balance = 1;

/// 1_000_000_000_000
pub const WEIGHT_PER_SECOND: u64 = 1_000_000_000_000;
/// 1_000_000_000
pub const WEIGHT_PER_MILLIS: u64 = WEIGHT_PER_SECOND / 1000;
/// 1_000_000
pub const WEIGHT_PER_MICROS: u64 = WEIGHT_PER_MILLIS / 1000;
/// 1_000
pub const WEIGHT_PER_NANOS: u64 = WEIGHT_PER_MICROS / 1000;

parameter_types! {
    /// By default, Substrate uses RocksDB, so this will be the weight used throughout
    /// the runtime.
    pub const RocksDbWeight: RuntimeDbWeight = RuntimeDbWeight {
        read: 25_000 * WEIGHT_PER_NANOS,
        write: 100_000 * WEIGHT_PER_NANOS,
    };
}

#[cfg(test)]
mod constants_tests {
    use super::*;

    #[test]
    fn sanity_check_rocksdb_weight() {
        use frame_support::weights::constants::RocksDbWeight as ImportedRocksDbWeight;
        assert_eq!(ImportedRocksDbWeight::get().read, RocksDbWeight::get().read);
        assert_eq!(
            ImportedRocksDbWeight::get().write,
            RocksDbWeight::get().write
        );
    }

    #[test]
    fn sanity_check_weight_per_time_constants() {
        use frame_support::weights::constants::{
            WEIGHT_REF_TIME_PER_MICROS as IMPORTED_WEIGHT_PER_MICROS,
            WEIGHT_REF_TIME_PER_MILLIS as IMPORTED_WEIGHT_PER_MILLIS,
            WEIGHT_REF_TIME_PER_NANOS as IMPORTED_WEIGHT_PER_NANOS,
            WEIGHT_REF_TIME_PER_SECOND as IMPORTED_WEIGHT_PER_SECOND,
        };

        assert_eq!(WEIGHT_PER_SECOND, IMPORTED_WEIGHT_PER_SECOND);
        assert_eq!(WEIGHT_PER_MILLIS, IMPORTED_WEIGHT_PER_MILLIS);
        assert_eq!(WEIGHT_PER_MICROS, IMPORTED_WEIGHT_PER_MICROS);
        assert_eq!(WEIGHT_PER_NANOS, IMPORTED_WEIGHT_PER_NANOS);
    }
}
