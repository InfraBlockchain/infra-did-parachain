#![cfg_attr(not(feature = "std"), no_std)]

use super::*;
#[allow(deprecated)]
use frame_support::migration::remove_storage_prefix;
use frame_support::{
    dispatch::GetStorageVersion,
    migration::{have_storage_value, storage_key_iter},
    pallet_prelude::Weight,
    traits::{Get, PalletInfoAccess, StorageVersion},
    Twox64Concat,
};

/// This migrates the pallet from the standard version by parity to our modified storage.
impl<T: Config> Pallet<T> {
    pub fn migrate_v0_to_v1() -> frame_support::weights::Weight {
        // Storage migrations should use storage versions for safety.
        if Self::on_chain_storage_version() < 1 {
            log::info!("Executing collator-selection V0->V1 migration!");

            // Drain all keys from the old (now unused) map
            let iter_map = storage_key_iter::<T::AccountId, T::BlockNumber, Twox64Concat>(
                Self::name().as_bytes(),
                b"LastAuthoredBlock",
            )
            .drain();
            let mut dropcount = 0;
            for _ in iter_map {
                dropcount += 1;
            }
            log::info!(" >>> Cleaned {} keys from LastAuthoredBlock", dropcount);
            #[allow(deprecated)]
            remove_storage_prefix(Self::name().as_bytes(), b"LastAuthoredBlock", &[]);
            log::info!(" >>> Removed LastAuthoredBlock from storage");

            // Update storage version.
            StorageVersion::new(1).put::<Self>();

            // Remove KickThreshold if customized
            if have_storage_value(Self::name().as_bytes(), b"KickThreshold", &[]) {
                #[allow(deprecated)]
                remove_storage_prefix(Self::name().as_bytes(), b"KickThreshold", &[]);
                log::info!(" >>> Removed KickThreshold");
            } else {
                log::warn!(" !!! Chain did not have KickThreshold in storage. This is unexpected but is possible if the genesis config was never changed");
            }

            // Return the weight consumed by the migration.
            T::DbWeight::get().reads_writes(1, dropcount + 1)
        } else {
            log::debug!("collator-selection V0->V1 migration not needed!");
            Weight::zero()
        }
    }
    pub fn pre_migrate_v0_to_v1() -> Result<(), &'static str> {
        let chainver = Self::on_chain_storage_version();
        if chainver >= 1 {
            return Err("Migration to V1 does not apply");
        }
        if !have_storage_value(Self::name().as_bytes(), b"KickThreshold", &[]) {
            log::warn!("Precheck: KickThreshold does not exist");
        }
        if storage_key_iter::<T::AccountId, T::BlockNumber, Twox64Concat>(
            Self::name().as_bytes(),
            b"LastAuthoredBlock",
        )
        .count()
            == 0
        {
            return Err("LastAuthoredBlock does not exist");
        }
        Ok(())
    }

    pub fn post_migrate_v0_to_v1() -> Result<(), &'static str> {
        if Self::on_chain_storage_version() != 1 {
            return Err("storage version not upgraded");
        }
        if have_storage_value(Self::name().as_bytes(), b"KickThreshold", &[]) {
            return Err("KickThreshold wasn't removed");
        }
        if storage_key_iter::<T::AccountId, T::BlockNumber, Twox64Concat>(
            Self::name().as_bytes(),
            b"LastAuthoredBlock",
        )
        .count()
            > 0
        {
            return Err("LastAuthoredBlock wasn't removed");
        }
        Ok(())
    }
}
