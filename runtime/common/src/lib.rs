#![allow(non_upper_case_globals)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::upper_case_acronyms)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod migration;

use common_primitives::{constants::WEIGHT_PER_NANOS, types::BlockNumber};
use frame_support::{parameter_types, weights::Weight};
use pallet_transaction_payment::{Multiplier, TargetedFeeAdjustment};
use sp_runtime::{traits::Bounded, FixedPointNumber, Perquintill};

// From https://github.com/paritytech/polkadot/pull/4332/files?diff=unified&w=1 @ runtime/common/src/lib.rs
/// Macro to set a value (e.g. when using the `parameter_types` macro) to either a production value
/// or to an environment variable or testing value (in case the `fast-runtime` feature is selected).
/// Note that the environment variable is evaluated _at compile time_.
///
/// Usage:
/// ```Rust
/// parameter_types! {
///     // Note that the env variable version parameter cannot be const.
///     pub LaunchPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1, "KSM_LAUNCH_PERIOD");
///     pub const VotingPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1 * MINUTES);
/// }
#[macro_export]
macro_rules! prod_or_fast {
    ($prod:expr, $test:expr) => {
        if cfg!(feature = "fast-runtime") {
            $test
        } else {
            $prod
        }
    };
    ($prod:expr, $test:expr, $env:expr) => {
        if cfg!(feature = "fast-runtime") {
            core::option_env!($env)
                .map(|s| s.parse().ok())
                .flatten()
                .unwrap_or($test)
        } else {
            $prod
        }
    };
}

parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
    /// The portion of the `NORMAL_DISPATCH_RATIO` that we adjust the fees with. Blocks filled less
    /// than this will decrease the weight and more will increase.
    pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);
    /// The adjustment variable of the runtime. Higher values will cause `TargetBlockFullness` to
    /// change the fees more rapidly.
    pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(3, 100_000);
    /// Minimum amount of the multiplier. This value cannot be too low. A test case should ensure
    /// that combined with `AdjustmentVariable`, we can recover from the minimum.
    /// See `multiplier_can_grow_from_zero`.
    pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000u128);
    pub MaximumMultiplier: Multiplier = Bounded::max_value();
}

/// Parameterized slow adjusting fee updated based on
/// https://research.web3.foundation/en/latest/polkadot/overview/2-token-economics.html#-2.-slow-adjusting-mechanism
pub type SlowAdjustingFeeUpdate<R> = TargetedFeeAdjustment<
    R,
    TargetBlockFullness,
    AdjustmentVariable,
    MinimumMultiplier,
    MaximumMultiplier,
>;

parameter_types! {
    /// Time to execute an empty block.
    /// Calculated by multiplying the *Average* with `1` and adding `0`.
    ///
    /// Stats nanoseconds:
    ///   Min, Max: 5_303_128, 5_507_784
    ///   Average:  5_346_284
    ///   Median:   5_328_139
    ///   Std-Dev:  41749.5
    ///
    /// Percentiles nanoseconds:
    ///   99th: 5_489_273
    ///   95th: 5_433_314
    ///   75th: 5_354_812
    pub const BlockExecutionWeight: Weight = Weight::from_ref_time(WEIGHT_PER_NANOS.saturating_mul(5_346_284));
}

parameter_types! {
    /// Time to execute a NO-OP extrinsic, for example `System::remark`.
    /// Calculated by multiplying the *Average* with `1` and adding `0`.
    ///
    /// Stats nanoseconds:
    ///   Min, Max: 86_060, 86_999
    ///   Average:  86_298
    ///   Median:   86_248
    ///   Std-Dev:  207.19
    ///
    /// Percentiles nanoseconds:
    ///   99th: 86_924
    ///   95th: 86_828
    ///   75th: 86_347
    pub const ExtrinsicBaseWeight: Weight = Weight::from_ref_time(WEIGHT_PER_NANOS.saturating_mul(86_298));
}

#[cfg(test)]
mod sanity_tests {
    use super::*;
    use frame_support::weights::constants::ExtrinsicBaseWeight as ImportedExtrinsicBaseWeight;

    #[test]
    fn sanity_check_extrinsic_base_weight() {
        assert_eq!(
            ExtrinsicBaseWeight::get(),
            ImportedExtrinsicBaseWeight::get()
        );
    }
}