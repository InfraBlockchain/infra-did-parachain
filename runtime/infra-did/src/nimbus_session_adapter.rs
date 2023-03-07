//! This file contains wrappers for Nimbus to make them compatible with pallet session traits,
//! but has No-Op implementations on everything so as to not change behavior of the running chain just yet

use crate::AccountId;
use frame_support::traits::OneSessionHandler;
use frame_system::Config;
use pallet_author_inherent::Pallet as AuthorInherent;
use session_key_primitives::vrf::VrfSessionKey;
use sp_application_crypto::BoundToRuntimeAppPublic;

/// This adapts pallet AuthorInherent to be compatible with pallet session
/// making it suitable as a SessionKey entry
pub struct AuthorInherentWithNoOpSession<T: Config>(pub AuthorInherent<T>);

impl<T: Config> BoundToRuntimeAppPublic for AuthorInherentWithNoOpSession<T> {
    type Public = <AuthorInherent<T> as BoundToRuntimeAppPublic>::Public;
}

impl<T: Config> OneSessionHandler<T::AccountId> for AuthorInherentWithNoOpSession<T> {
    type Key = <AuthorInherent<T> as BoundToRuntimeAppPublic>::Public;

    fn on_genesis_session<'a, I: 'a>(_: I)
    where
        I: Iterator<Item = (&'a T::AccountId, Self::Key)>,
    {
    }

    fn on_new_session<'a, I: 'a>(_: bool, _: I, _: I)
    where
        I: Iterator<Item = (&'a T::AccountId, Self::Key)>,
    {
    }

    fn on_disabled(_: u32) {}

    fn on_before_session_ending() {}
}

/// This adapts VrfSessionKey to be compatible with pallet session
/// making it suitable as a SessionKey entry
pub struct VrfWithNoOpSession(pub VrfSessionKey);

impl BoundToRuntimeAppPublic for VrfWithNoOpSession {
    type Public = <VrfSessionKey as BoundToRuntimeAppPublic>::Public;
}

impl OneSessionHandler<AccountId> for VrfWithNoOpSession {
    type Key = <VrfSessionKey as BoundToRuntimeAppPublic>::Public;

    fn on_genesis_session<'a, I: 'a>(_: I)
    where
        I: Iterator<Item = (&'a AccountId, Self::Key)>,
    {
    }

    fn on_new_session<'a, I: 'a>(_: bool, _: I, _: I)
    where
        I: Iterator<Item = (&'a AccountId, Self::Key)>,
    {
    }

    fn on_disabled(_: u32) {}

    fn on_before_session_ending() {}
}
