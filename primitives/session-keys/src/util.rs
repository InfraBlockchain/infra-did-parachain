//! Key Generation Utilities

use common_primitives::types::{AccountId, Signer};
use sp_core::{crypto::CryptoType, Pair};
use sp_runtime::traits::IdentifyAccount;

/// Public Key Type
pub type PublicKey<T> = <<T as CryptoType>::Pair as Pair>::Public;

/// Derives [`PublicKey`] from `seed` for the corresponding crypto type `T` without checking that
/// the `seed` is valid.
#[inline]
pub fn unchecked_public_key<T>(seed: &str) -> PublicKey<T>
where
    T: CryptoType,
{
    T::Pair::from_string(&format!("//{seed}"), None)
        .expect("The validity of the seed is unchecked.")
        .public()
}

/// Derives [`AccountId`] from `seed` for the corresponding crypto type `T` without checking that
/// the `seed` is valid.
#[inline]
pub fn unchecked_account_id<T>(seed: &str) -> AccountId
where
    T: CryptoType,
    Signer: From<PublicKey<T>>,
{
    Signer::from(unchecked_public_key::<T>(seed)).into_account()
}
