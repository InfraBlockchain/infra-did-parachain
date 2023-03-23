//! Boilerplate for runtime module unit tests

use crate::{
    accumulator, anchor, attest, bbs_plus, blob,
    did::{self, Did, DidKey, DidSignature},
    keys_and_sigs, revoke, trusted_entity, util, StateChange, ToStateChange,
};

use crate::{
    keys_and_sigs::SigValue,
    revoke::{Policy, RegistryId, RevokeId},
    trusted_entity::Policy as TrustedEntityPolicy,
};
use codec::{Decode, Encode};
use frame_support::{
    parameter_types,
    traits::{Contains, OnFinalize, OnInitialize},
    weights::Weight,
};
use frame_system as system;
pub use rand::random;
use sp_core::{sr25519, Pair, H256};
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, ConstU32, IdentityLookup},
};
pub use std::iter::once;

// Configure a mock runtime to test the pallet.
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        DIDModule: did::{Pallet, Call, Storage, Event, Config},
        RevoMod: revoke::{Pallet, Call, Storage, Event},
        BlobMod: blob::{Pallet, Call, Storage},
        AnchorMod: anchor::{Pallet, Call, Storage, Event<T>},
        AttestMod: attest::{Pallet, Call, Storage},
        BBSPlusMod: bbs_plus::{Pallet, Call, Storage, Event},
        AccumMod: accumulator::{Pallet, Call, Storage, Event},
        TrustedEntityMod: trusted_entity::{Pallet, Call, Storage, Event},
    }
);

#[derive(Encode, Decode, scale_info_derive::TypeInfo, Clone, PartialEq, Debug, Eq)]
pub enum TestEvent {
    Did(crate::did::Event),
    Revoke(crate::revoke::Event),
    Anchor(crate::anchor::Event<Test>),
    Unknown,
    BBSPlus(bbs_plus::Event),
    Accum(accumulator::Event),
    TrustedEntity(trusted_entity::Event),
}

impl From<system::Event<Test>> for TestEvent {
    fn from(_: system::Event<Test>) -> Self {
        unimplemented!()
    }
}

impl From<pallet_balances::Event<Test>> for TestEvent {
    fn from(_: pallet_balances::Event<Test>) -> Self {
        unimplemented!()
    }
}

impl From<()> for TestEvent {
    fn from((): ()) -> Self {
        Self::Unknown
    }
}

impl From<crate::did::Event> for TestEvent {
    fn from(other: crate::did::Event) -> Self {
        Self::Did(other)
    }
}

impl From<crate::revoke::Event> for TestEvent {
    fn from(other: crate::revoke::Event) -> Self {
        Self::Revoke(other)
    }
}

impl From<crate::anchor::Event<Test>> for TestEvent {
    fn from(other: crate::anchor::Event<Test>) -> Self {
        Self::Anchor(other)
    }
}

impl From<bbs_plus::Event> for TestEvent {
    fn from(other: bbs_plus::Event) -> Self {
        Self::BBSPlus(other)
    }
}

impl From<accumulator::Event> for TestEvent {
    fn from(other: accumulator::Event) -> Self {
        Self::Accum(other)
    }
}

impl From<trusted_entity::Event> for TestEvent {
    fn from(other: trusted_entity::Event) -> Self {
        Self::TrustedEntity(other)
    }
}

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaxControllers: u32 = 15;
    pub const ByteReadWeight: Weight = Weight::from_ref_time(10);
}

pub struct BaseFilter;
impl Contains<RuntimeCall> for BaseFilter {
    fn contains(call: &RuntimeCall) -> bool {
        match call {
            _ => true,
        }
    }
}

impl system::Config for Test {
    type OnSetCode = ();
    type MaxConsumers = ConstU32<10>;
    type BaseCallFilter = BaseFilter;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = TestEvent;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type BlockWeights = ();
    type BlockLength = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ();
    type WeightInfo = ();
}

impl pallet_balances::Config for Test {
    type ReserveIdentifier = ();
    type MaxReserves = ();
    type MaxLocks = ();
    type Balance = u64;
    type RuntimeEvent = TestEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ();
    type AccountStore = System;
    type WeightInfo = ();
}

impl crate::did::Config for Test {
    type RuntimeEvent = TestEvent;
    type MaxDidDocRefSize = MaxDidDocRefSize;
    type DidDocRefPerByteWeight = DidDocRefPerByteWeight;
    type MaxServiceEndpointIdSize = MaxServiceEndpointIdSize;
    type ServiceEndpointIdPerByteWeight = ServiceEndpointIdPerByteWeight;
    type MaxServiceEndpointOrigins = MaxServiceEndpointOrigins;
    type MaxServiceEndpointOriginSize = MaxServiceEndpointOriginSize;
    type ServiceEndpointOriginPerByteWeight = ServiceEndpointOriginPerByteWeight;
}

impl crate::revoke::Config for Test {
    type RuntimeEvent = TestEvent;
    type MaxControllers = MaxControllers;
}

impl crate::trusted_entity::Config for Test {
    type RuntimeEvent = TestEvent;
    type MaxControllers = MaxControllers;
}

parameter_types! {
    pub const MaxBlobSize: u32 = 1024;
    pub const StorageWeight: Weight = Weight::from_ref_time(1100);
    pub const LabelMaxSize: u32 = 512;
    pub const LabelPerByteWeight: Weight = Weight::from_ref_time(10);
    pub const ParamsMaxSize: u32 = 512;
    pub const ParamsPerByteWeight: Weight = Weight::from_ref_time(10);
    pub const PublicKeyMaxSize: u32 = 128;
    pub const PublicKeyPerByteWeight: Weight = Weight::from_ref_time(10);
    pub const AccumulatedMaxSize: u32 = 256;
    pub const AccumulatedPerByteWeight: Weight = Weight::from_ref_time(10);
    pub const MaxDidDocRefSize: u16 = 128;
    pub const DidDocRefPerByteWeight: Weight = Weight::from_ref_time(10);
    pub const MaxServiceEndpointIdSize: u16 = 256;
    pub const ServiceEndpointIdPerByteWeight: Weight = Weight::from_ref_time(10);
    pub const MaxServiceEndpointOrigins: u16 = 20;
    pub const MaxServiceEndpointOriginSize: u16 = 256;
    pub const ServiceEndpointOriginPerByteWeight: Weight = Weight::from_ref_time(10);
}

impl crate::anchor::Config for Test {
    type RuntimeEvent = TestEvent;
}

impl crate::blob::Config for Test {
    type MaxBlobSize = MaxBlobSize;
    type StorageWeight = StorageWeight;
}

impl crate::attest::Config for Test {
    type StorageWeight = StorageWeight;
}

impl bbs_plus::Config for Test {
    type RuntimeEvent = TestEvent;
    type LabelMaxSize = LabelMaxSize;
    type LabelPerByteWeight = LabelPerByteWeight;
    type ParamsMaxSize = ParamsMaxSize;
    type ParamsPerByteWeight = ParamsPerByteWeight;
    type PublicKeyMaxSize = PublicKeyMaxSize;
    type PublicKeyPerByteWeight = PublicKeyPerByteWeight;
}

impl accumulator::Config for Test {
    type RuntimeEvent = TestEvent;
    type LabelMaxSize = LabelMaxSize;
    type LabelPerByteWeight = LabelPerByteWeight;
    type ParamsMaxSize = ParamsMaxSize;
    type ParamsPerByteWeight = ParamsPerByteWeight;
    type PublicKeyMaxSize = PublicKeyMaxSize;
    type PublicKeyPerByteWeight = PublicKeyPerByteWeight;
    type AccumulatedMaxSize = AccumulatedMaxSize;
    type AccumulatedPerByteWeight = AccumulatedPerByteWeight;
}

pub const ABBA: u64 = 0;
pub const DIDA: Did = Did([0u8; 32]);
pub const DIDB: Did = Did([1u8; 32]);
pub const DIDC: Did = Did([2u8; 32]);
pub const RGA: RegistryId = [0u8; 32];
pub const RA: RevokeId = [0u8; 32];
pub const RB: RevokeId = [1u8; 32];
pub const RC: RevokeId = [2u8; 32];

/// check whether test externalities are available
pub fn in_ext() -> bool {
    std::panic::catch_unwind(|| sp_io::storage::exists(&[])).is_ok()
}

#[test]
pub fn meta_in_ext() {
    assert!(!in_ext());
    ext().execute_with(|| assert!(in_ext()));
}

pub fn ext() -> sp_io::TestExternalities {
    let mut ret: sp_io::TestExternalities = system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into();
    ret.execute_with(|| {
        system::Pallet::<Test>::initialize(
            &1, // system module will not store events if block_number == 0
            &[0u8; 32].into(),
            &Default::default(),
        );
    });
    ret
}

/// create a OneOf policy
pub fn oneof(dids: &[Did]) -> Policy {
    Policy::OneOf(dids.iter().cloned().collect())
}

/// create a OneOf policy
pub fn trusted_entity_oneof(dids: &[Did]) -> TrustedEntityPolicy {
    TrustedEntityPolicy::OneOf(dids.iter().cloned().collect())
}

/// generate a random keypair
pub fn gen_kp() -> sr25519::Pair {
    sr25519::Pair::generate_with_phrase(None).0
}

// Create did for `did`. Return the randomly generated signing key.
// The did public key is controlled by some non-existent account (normally a security
// concern), but that doesn't matter for our purposes.
pub fn create_did(did: did::Did) -> sr25519::Pair {
    let kp = gen_kp();
    println!("did pk: {:?}", kp.public().0);
    did::Pallet::<Test>::new_onchain(
        RuntimeOrigin::signed(ABBA),
        did,
        vec![
            DidKey::new_with_all_relationships(keys_and_sigs::PublicKey::Sr25519(util::Bytes32 {
                value: kp.public().0,
            }))
            .into(),
        ],
        vec![].into_iter().collect(),
    )
    .unwrap();
    kp
}

/// create a did with a random id and random signing key
pub fn newdid() -> (Did, sr25519::Pair) {
    let d: Did = Did(rand::random());
    (d, create_did(d))
}

pub fn sign<T: frame_system::Config>(
    payload: &StateChange<T>,
    keypair: &sr25519::Pair,
) -> SigValue {
    SigValue::Sr25519(util::Bytes64 {
        value: keypair.sign(&payload.encode()).0,
    })
}

pub fn did_sig<T: frame_system::Config, A: ToStateChange<T>, D: Into<Did>>(
    change: &A,
    keypair: &sr25519::Pair,
    did: D,
    key_id: u32,
) -> DidSignature<D> {
    let sig = sign(&change.to_state_change(), keypair);
    DidSignature {
        did,
        key_id: key_id.into(),
        sig,
    }
}

pub fn did_sig_on_bytes<T: frame_system::Config, D: Into<Did>>(
    msg_bytes: &[u8],
    keypair: &sr25519::Pair,
    did: D,
    key_id: u32,
) -> DidSignature<D> {
    let sig = SigValue::Sr25519(util::Bytes64 {
        value: keypair.sign(msg_bytes).0,
    });

    DidSignature {
        did,
        key_id: key_id.into(),
        sig,
    }
}

/// create a random byte array with set len
pub fn random_bytes(len: usize) -> Vec<u8> {
    (0..len).map(|_| rand::random()).collect()
}

/// Changes the block number. Calls `on_finalize` and `on_initialize`
pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        if System::block_number() > 1 {
            System::on_finalize(System::block_number());
        }
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
    }
}

pub fn check_nonce(d: &Did, nonce: u64) {
    let did_detail = DIDModule::onchain_did_details(&d).unwrap();
    assert_eq!(did_detail.nonce, nonce);
}

pub fn inc_nonce(d: &Did) {
    let mut did_detail = DIDModule::onchain_did_details(&d).unwrap();
    did_detail.nonce = did_detail.next_nonce().unwrap();
    DIDModule::insert_did_details(*d, did_detail);
}
