use crate as dock;
use crate::{
    did::{self, Did, DidSignature},
    util::{NonceError, WithNonce},
    Action, StorageVersion, ToStateChange,
};
use alloc::collections::BTreeSet;
use codec::{Decode, Encode};
use core::{fmt::Debug, marker::PhantomData};
use sp_std::vec::Vec;

pub use actions::*;
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
    traits::Get,
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::traits::Hash;
use sp_std::prelude::*;
// use weights::*;

mod actions;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarks;
mod r#impl;
#[cfg(test)]
pub mod tests;
// mod weights;

pub trait Config: system::Config + did::Config {
    type RuntimeEvent: From<Event> + Into<<Self as system::Config>::RuntimeEvent>;
    type MaxControllers: Get<u32>;
}

pub type AuthorizerId = [u8; 32];

pub type TrustedEntityId = [u8; 32];

/// Collection of signatures sent by different DIDs.
#[derive(PartialEq, Eq, Encode, Decode, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(scale_info_derive::TypeInfo)]
#[scale_info(skip_type_params(T))]
#[scale_info(omit_prefix)]
pub struct DidSigs<T>
where
    T: frame_system::Config,
{
    /// Signature by DID
    pub sig: DidSignature<Did>,
    /// Nonce used to make the above signature
    pub nonce: T::BlockNumber,
}

/// Authorization logic for a authorizer.
#[derive(PartialEq, Eq, Encode, Decode, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(scale_info_derive::TypeInfo)]
#[scale_info(omit_prefix)]
pub enum Policy {
    /// Set of dids allowed to modify a authorizer.
    OneOf(BTreeSet<Did>),
}

impl Default for Policy {
    fn default() -> Self {
        Self::OneOf(Default::default())
    }
}

impl Policy {
    /// Check for user error in the construction of self.
    /// if self is invalid, return `false`, else return `true`.
    fn valid(&self) -> bool {
        self.len() != 0
    }

    fn len(&self) -> u32 {
        match self {
            Self::OneOf(controllers) => controllers.len() as u32,
        }
    }
}

/// Metadata about a trusted entity scope.
#[derive(PartialEq, Eq, Encode, Decode, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(scale_info_derive::TypeInfo)]
#[scale_info(omit_prefix)]
pub struct Authorizer {
    /// Who is allowed to update this authorizer.
    pub policy: Policy,
    pub add_only: bool,
}

decl_event!(
    pub enum Event {
        /// Authorizer with given id created
        AuthorizerAdded(AuthorizerId),
        /// Some items were added from given authorizer id
        AddedInTrustedEntity(AuthorizerId),
        /// Some items were removed from given authorizer id
        RemovedInTrustedEntity(AuthorizerId),
        /// Authorizer with given id removed
        AuthorizerRemoved(AuthorizerId),
    }
);

decl_error! {
    pub enum TrustError for Module<T: Config> where T: Debug {
        /// The authorization policy provided was illegal.
        InvalidPolicy,
        /// Proof of authorization does not meet policy requirements.
        NotAuthorized,
        /// A authorizer with that name already exists.
        AuthorizerExists,
        /// A authorizer with that name does not exist.
        NoAuthorizer,
        /// nonce is incorrect. This is related to replay protection.
        IncorrectNonce,
        /// Too many controllers specified.
        TooManyControllers,
        /// This authorizer is marked as add_only. Deletion of trusted entities is not allowed. Deletion of
        /// the authorizer is not allowed.
        AddOnly,
        /// Action is empty.
        EmptyPayload
    }
}

impl<T: Config + Debug> From<NonceError> for TrustError<T> {
    fn from(NonceError::IncorrectNonce: NonceError) -> Self {
        Self::IncorrectNonce
    }
}

decl_storage! {
    trait Store for Module<T: Config> as TrustedEntity where T: Debug {
        pub(crate) Authorizers get(fn get_authorizer):
            map hasher(blake2_128_concat) dock::trusted_entity::AuthorizerId => Option<Authorizer>;

        TrustedEntities get(fn get_trusted_entity):
            double_map hasher(blake2_128_concat) dock::trusted_entity::AuthorizerId, hasher(opaque_blake2_256) dock::trusted_entity::TrustedEntityId => Option<()>;

        pub Version get(fn version): StorageVersion;
    }
    add_extra_genesis {
        build(|_| {
            Version::put(StorageVersion::MultiKey);
        })
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::RuntimeOrigin, T: Debug {
        fn deposit_event() = default;

        type Error = TrustError<T>;

        /// Create a new authrorizer named `id` with `authrorizer` metadata.
        ///
        /// # Errors
        ///
        /// Returns an error if `id` is already in use as a authrorizer id.
        ///
        /// Returns an error if `authrorizer.policy` is invalid.
        #[weight = 1]
        pub fn new_authorizer(
            origin,
            add_authorizer: AddAuthorizer
        ) -> DispatchResult {
            ensure_signed(origin)?;

            Self::new_authorizer_(add_authorizer)?;
            Ok(())
        }

        /// Create some trusted entities according to the `entity`` command.
        ///
        /// # Errors
        ///
        /// Returns an error if `entity.last_modified` does not match the block number when the
        /// authorizer referenced by `entity.authorizer_id` was last modified.
        ///
        /// Returns an error if `proof` does not satisfy the policy requirements of the authorizer
        /// referenced by `entity.authorizer_id`.
        #[weight = 1]
        pub fn add_trusted_entity(
            origin,
            entity: dock::trusted_entity::AddTrustedEntityRaw<T>,
            proof: Vec<DidSigs<T>>,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            Self::try_exec_action_over_authorizer(entity, proof, Self::add_trusted_entity_)?;
            Ok(())
        }

        /// Delete some trusted entities according to the `entity` command.
        ///
        /// # Errors
        ///
        /// Returns an error if the authorizer referenced by `entity.authorizer_id` is `add_only`.
        ///
        /// Returns an error if `entity.last_modified` does not match the block number when the
        /// authorizer referenced by `authrorizer.authorizer_id` was last modified.
        ///
        /// Returns an error if `proof` does not satisfy the policy requirements of the authorizer
        /// referenced by `entity.authorizer_id`.
        #[weight = 1]
        pub fn remove_trusted_entity(
            origin,
            entity: dock::trusted_entity::RemoveTrustedEntityRaw<T>,
            proof: Vec<DidSigs<T>>,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            Self::try_exec_action_over_authorizer(entity, proof, Self::remove_trusted_entity_)?;
            Ok(())
        }

        /// Delete an entire authorizer. Deletes all trusted entities within the authorizer, as well as
        /// authorizer metadata. Once the authorizer is deleted, it can be reclaimed by any party using
        /// a call to `new_authorizer`.
        ///
        /// # Errors
        ///
        /// Returns an error if the authorizer referenced by `entity.authorizer_id` is `add_only`.
        ///
        /// Returns an error if `removal.last_modified` does not match the block number when the
        /// authorizer referenced by `removal.authorizer_id` was last modified.
        ///
        /// Returns an error if `proof` does not satisfy the policy requirements of the authorizer
        /// referenced by `removal.authorizer_id`.
        #[weight = 1]
        pub fn remove_authorizer(
            origin,
            removal: dock::trusted_entity::RemoveAuthorizerRaw<T>,
            proof: Vec<DidSigs<T>>,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            Self::try_exec_removable_action_over_authorizer(removal, proof, Self::remove_authorizer_)?;
            Ok(())
        }
    }
}
