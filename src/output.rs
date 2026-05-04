use std::collections::HashMap;

pub use psbt_v2::v2::Output;

use crate::collections::vec::VecWrapper;
use crate::partial_join::JoinResult;
use crate::partial_join::PartialJoin;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputSet(HashMap<Vec<u8>, Output>);

impl OutputSet {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
impl PartialJoin for OutputSet {
    type Error = <HashMap<Vec<u8>, Output> as PartialJoin>::Error;

    fn try_join(self, other: Self) -> JoinResult<Self> {
        match self.0.try_join(other.0) {
            Ok(lub) => Ok(OutputSet(lub)),
            Err(e) => Err(e),
        }
    }
}

impl PartialJoin for Output {
    type Error = ResultOutput;

    fn try_join(self, other: Self) -> JoinResult<Self> {
        ResultOutput {
            amount: self.amount.try_join(other.amount),
            script_pubkey: self.script_pubkey.try_join(other.script_pubkey), // FIXME allow empty to non-empty to behave like Option<ScriptBuf> instead of ScriptBuf under equality
            redeem_script: self.redeem_script.try_join(other.redeem_script),
            witness_script: self.witness_script.try_join(other.witness_script),
            tap_internal_key: self.tap_internal_key.try_join(other.tap_internal_key),
            tap_tree: self.tap_tree.try_join(other.tap_tree),
            bip32_derivations: self.bip32_derivations.try_join(other.bip32_derivations),
            tap_key_origins: self.tap_key_origins.try_join(other.tap_key_origins),
            proprietaries: self.proprietaries.try_join(other.proprietaries),
            unknowns: self.unknowns.try_join(other.unknowns),
        }
        .ok()
    }
}

impl FromIterator<Output> for OutputSet {
    fn from_iter<T: IntoIterator<Item = Output>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|output| (output.unique_id(), output))
                .collect(),
        )
    }
}

impl IntoIterator for OutputSet {
    type Item = Output;
    type IntoIter = std::collections::hash_map::IntoValues<Vec<u8>, Output>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_values()
    }
}

trait OutputExt {
    fn unique_id(&self) -> Vec<u8>;
}

impl OutputExt for Output {
    fn unique_id(&self) -> Vec<u8> {
        // FIXME use proprietary PSBT_OUT_UNIQUE_ID field not PSBT_OUT_SCRIPT
        self.script_pubkey.to_bytes()
    }
}

impl FromIterator<Output> for VecWrapper<Output> {
    fn from_iter<T: IntoIterator<Item = Output>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for VecWrapper<Output> {
    type Item = Output;
    type IntoIter = std::vec::IntoIter<Output>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

mod result {
    pub use std::collections::BTreeMap;

    use bitcoin::bip32::KeySource;
    use bitcoin::key::XOnlyPublicKey;
    use bitcoin::taproot::{TapLeafHash, TapTree};
    use bitcoin::{secp256k1, Amount, ScriptBuf};

    use psbt_v2::raw;

    use crate::partial_join::JoinResult;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ResultOutput {
        /// The output's amount (serialized as satoshis).
        pub amount: JoinResult<Amount>,

        /// The script for this output, also known as the scriptPubKey.
        pub script_pubkey: JoinResult<ScriptBuf>,

        /// The redeem script for this output.
        pub redeem_script: JoinResult<Option<ScriptBuf>>,
        /// The witness script for this output.
        pub witness_script: JoinResult<Option<ScriptBuf>>,
        /// A map from public keys needed to spend this output to their
        /// corresponding master key fingerprints and derivation paths.
        pub bip32_derivations: JoinResult<BTreeMap<secp256k1::PublicKey, KeySource>>,
        /// The internal pubkey.
        pub tap_internal_key: JoinResult<Option<XOnlyPublicKey>>,
        /// Taproot Output tree.
        pub tap_tree: JoinResult<Option<TapTree>>,
        /// Map of tap root x only keys to origin info and leaf hashes contained in it.
        pub tap_key_origins: JoinResult<BTreeMap<XOnlyPublicKey, (Vec<TapLeafHash>, KeySource)>>,
        /// Proprietary key-value pairs for this output.
        pub proprietaries: JoinResult<BTreeMap<raw::ProprietaryKey, Vec<u8>>>,
        /// Unknown key-value pairs for this output.
        pub unknowns: JoinResult<BTreeMap<raw::Key, Vec<u8>>>,
    }
}

pub use result::ResultOutput;

impl ResultOutput {
    fn ok(self) -> Result<Output, ResultOutput> {
        match self {
            ResultOutput {
                amount: Ok(amount),
                script_pubkey: Ok(script_pubkey), // FIXME allow empty to non-empty to behave like Option<ScriptBuf> instead of ScriptBuf under equality
                redeem_script: Ok(redeem_script),
                witness_script: Ok(witness_script),
                tap_internal_key: Ok(tap_internal_key),
                tap_tree: Ok(tap_tree),
                bip32_derivations: Ok(bip32_derivations),
                tap_key_origins: Ok(tap_key_origins),
                proprietaries: Ok(proprietaries),
                unknowns: Ok(unknowns),
            } => Ok(Output {
                amount,
                script_pubkey,
                redeem_script,
                witness_script,
                tap_internal_key,
                tap_tree,
                bip32_derivations,
                tap_key_origins,
                proprietaries,
                unknowns,
            }),
            _ => Err(self),
        }
    }
}

#[test]
fn test_output_set() {
    // InputSet(todo!())
}
