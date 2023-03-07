//! Types

use sp_core::H256;
use sp_runtime::{
    generic,
    traits::{BlakeTwo256, IdentifyAccount, Verify},
    MultiSignature, OpaqueExtrinsic,
};

/// Block Number Type
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Signer Type
pub type Signer = <Signature as Verify>::Signer;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <Signer as IdentifyAccount>::AccountId;

/// Account Index Type
///
/// This index is used to look up accounts.
pub type AccountIndex = u32;

/// Asset Id Type
pub type CommonAssetId = u128;

/// Balance of an Account
pub type Balance = u128;

/// Transaction Index Type
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = H256;

/// Block Header Type
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block Type
pub type Block = generic::Block<Header, OpaqueExtrinsic>;

/// Digest Item Type
pub type DigestItem = generic::DigestItem;

/// Moment Type
pub type Moment = u64;
