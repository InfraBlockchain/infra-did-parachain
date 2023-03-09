use super::*;

#[derive(PartialEq, Eq, Encode, Decode, scale_info_derive::TypeInfo, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[scale_info(omit_prefix)]
pub struct AddAuthorizer {
    pub id: AuthorizerId,
    pub new_authorizer: Authorizer,
}

/// Command to create a set of trusted entities withing a authorizer.
/// Creation of trusted entities is idempotent; creating a trusted entities that already exists is allowed,
/// but has no effect.
#[derive(PartialEq, Eq, Encode, Decode, scale_info_derive::TypeInfo, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[scale_info(skip_type_params(T))]
#[scale_info(omit_prefix)]
pub struct AddTrustedEntityRaw<T> {
    /// The authorizer on which to operate
    pub authorizer_id: AuthorizerId,
    /// entity ids which will be added
    pub entity_ids: BTreeSet<TrustedEntityId>,
    #[codec(skip)]
    #[cfg_attr(feature = "serde", serde(skip))]
    pub _marker: PhantomData<T>,
}

/// Command to remove a set of trusted entities within a authorizer.
/// Removal of trusted entities is idempotent; removing a trusted entities that doesn't exists is allowed,
/// but has no effect.
#[derive(PartialEq, Eq, Encode, Decode, scale_info_derive::TypeInfo, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[scale_info(skip_type_params(T))]
#[scale_info(omit_prefix)]
pub struct RemoveTrustedEntityRaw<T> {
    /// The authorizer on which to operate
    pub authorizer_id: AuthorizerId,
    /// entity ids which will be added
    pub entity_ids: BTreeSet<TrustedEntityId>,
    #[codec(skip)]
    #[cfg_attr(feature = "serde", serde(skip))]
    pub _marker: PhantomData<T>,
}

/// Command to remove an entire authorizer. Removes all trusted entities in the authorizer as well as
/// authorizer metadata.
#[derive(PartialEq, Eq, Encode, Decode, scale_info_derive::TypeInfo, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[scale_info(skip_type_params(T))]
#[scale_info(omit_prefix)]
pub struct RemoveAuthorizerRaw<T> {
    /// The authorizer on which to operate
    pub authorizer_id: AuthorizerId,
    #[codec(skip)]
    #[cfg_attr(feature = "serde", serde(skip))]
    pub _marker: PhantomData<T>,
}

crate::impl_action! {
    for AuthorizerId:
        AddTrustedEntityRaw with entity_ids.len() as len, authorizer_id as target no_state_change,
        RemoveTrustedEntityRaw with entity_ids.len() as len, authorizer_id as target no_state_change,
        RemoveAuthorizerRaw with 1 as len, authorizer_id as target no_state_change
}

/// Command to create a set of trusted entitiess withing a authorizer.
/// Creation of trusted entitiess is idempotent; creating a trusted entities that already exists is allowed,
/// but has no effect.
pub type AddTrustedEntity<T> = WithNonce<T, AddTrustedEntityRaw<T>>;
/// Command to remove a set of trusted entitiess within a authorizer.
/// Removal of trusted entitiess is idempotent; removing a trusted entities that doesn't exists is allowed,
/// but has no effect.
pub type RemoveTrustedEntity<T> = WithNonce<T, RemoveTrustedEntityRaw<T>>;
/// Command to remove an entire authorizer. Removes all trusted entitiess in the authorizer as well as
/// authorizer metadata.
pub type RemoveAuthorizer<T> = WithNonce<T, RemoveAuthorizerRaw<T>>;

crate::impl_action_with_nonce! {
    for AuthorizerId:
    AddTrustedEntity with data().len() as len, data().authorizer_id as target,
    RemoveTrustedEntity with data().len() as len, data().authorizer_id as target,
    RemoveAuthorizer with data().len() as len, data().authorizer_id as target
}
