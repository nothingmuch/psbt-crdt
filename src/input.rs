use std::collections::HashMap;

use bitcoin::OutPoint;
pub use psbt_v2::v2::Input;

use crate::collections::vec::VecWrapper;
use crate::partial_join::{JoinResult, PartialJoin};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputSet(HashMap<OutPoint, Input>);

impl FromIterator<Input> for InputSet {
    fn from_iter<T: IntoIterator<Item = Input>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|input| (input.out_point(), input))
                .collect(),
        )
    }
}

impl IntoIterator for InputSet {
    type Item = Input;
    type IntoIter = std::collections::hash_map::IntoValues<OutPoint, Input>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_values()
    }
}

impl InputSet {
    pub fn spends_outpoint(&self, outpoint: &OutPoint) -> bool {
        self.0.contains_key(outpoint)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl PartialJoin for InputSet {
    type Error = <HashMap<OutPoint, Input> as PartialJoin>::Error;
    // type Error = HashMap<OutPoint, Result<Input, (Input, Input)>>;

    fn try_join(self, other: Self) -> JoinResult<Self> {
        match self.0.try_join(other.0) {
            Ok(lub) => Ok(InputSet(lub)),
            Err(e) => Err(e),
        }
    }
}

impl PartialJoin for Input {
    type Error = ResultInput;

    fn try_join(self, other: Self) -> JoinResult<Self> {
        ResultInput {
            previous_txid: self.previous_txid.try_join(other.previous_txid),
            spent_output_index: self.spent_output_index.try_join(other.spent_output_index),
            sequence: self.sequence.try_join(other.sequence),
            min_time: self.min_time.try_join(other.min_time),
            min_height: self.min_height.try_join(other.min_height),
            non_witness_utxo: self.non_witness_utxo.try_join(other.non_witness_utxo),
            witness_utxo: self.witness_utxo.try_join(other.witness_utxo),
            partial_sigs: self.partial_sigs.try_join(other.partial_sigs),
            sighash_type: self.sighash_type.try_join(other.sighash_type),
            redeem_script: self.redeem_script.try_join(other.redeem_script),
            witness_script: self.witness_script.try_join(other.witness_script),
            bip32_derivations: self.bip32_derivations.try_join(other.bip32_derivations),
            final_script_sig: self.final_script_sig.try_join(other.final_script_sig),
            final_script_witness: self
                .final_script_witness
                .try_join(other.final_script_witness),
            ripemd160_preimages: self.ripemd160_preimages.try_join(other.ripemd160_preimages),
            sha256_preimages: self.sha256_preimages.try_join(other.sha256_preimages),
            hash160_preimages: self.hash160_preimages.try_join(other.hash160_preimages),
            hash256_preimages: self.hash256_preimages.try_join(other.hash256_preimages),
            tap_key_sig: self.tap_key_sig.try_join(other.tap_key_sig),
            tap_script_sigs: self.tap_script_sigs.try_join(other.tap_script_sigs),
            tap_scripts: self.tap_scripts.try_join(other.tap_scripts),
            tap_key_origins: self.tap_key_origins.try_join(other.tap_key_origins),
            tap_internal_key: self.tap_internal_key.try_join(other.tap_internal_key),
            tap_merkle_root: self.tap_merkle_root.try_join(other.tap_merkle_root),
            proprietaries: self.proprietaries.try_join(other.proprietaries),
            unknowns: self.unknowns.try_join(other.unknowns),
        }
        .ok()
    }
}

// Input::out_point() exists but is private so we reimeplemtn it here
trait InputExt {
    fn out_point(&self) -> OutPoint;
}

impl InputExt for Input {
    fn out_point(self: &Input) -> OutPoint {
        OutPoint {
            txid: self.previous_txid,
            vout: self.spent_output_index,
        }
    }
}

impl FromIterator<Input> for VecWrapper<Input> {
    fn from_iter<T: IntoIterator<Item = Input>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for VecWrapper<Input> {
    type Item = Input;
    type IntoIter = std::vec::IntoIter<Input>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

mod result {
    pub use std::collections::BTreeMap;

    use bitcoin::bip32::KeySource;
    use bitcoin::hashes::{hash160, ripemd160, sha256, sha256d};
    use bitcoin::key::{PublicKey, XOnlyPublicKey};
    use bitcoin::locktime::absolute;
    use bitcoin::taproot::{ControlBlock, LeafVersion, TapLeafHash, TapNodeHash};
    use bitcoin::{
        ecdsa, secp256k1, taproot, ScriptBuf, Sequence, Transaction, TxOut, Txid, Witness,
    };

    use psbt_v2::raw;
    use psbt_v2::PsbtSighashType;

    use crate::partial_join::JoinResult;

    // TODO impl Join for ResultInput
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ResultInput {
        /// The txid of the previous transaction whose output at `self.spent_output_index` is being spent.
        ///
        /// In other words, the output being spent by this `Input` is:
        ///
        ///  `OutPoint { txid: self.previous_txid, vout: self.spent_output_index }`
        pub previous_txid: JoinResult<Txid>,

        /// The index of the output being spent in the transaction with the txid of `self.previous_txid`.
        pub spent_output_index: JoinResult<u32>,

        /// The sequence number of this input.
        ///
        /// If omitted, assumed to be the final sequence number ([`Sequence::MAX`]).
        pub sequence: JoinResult<Option<Sequence>>,

        /// The minimum Unix timestamp that this input requires to be set as the transaction's lock time.
        pub min_time: JoinResult<Option<absolute::Time>>,

        /// The minimum block height that this input requires to be set as the transaction's lock time.
        pub min_height: JoinResult<Option<absolute::Height>>,

        /// The non-witness transaction this input spends from. Should only be
        /// `Option::Some` for inputs which spend non-segwit outputs or
        /// if it is unknown whether an input spends a segwit output.
        pub non_witness_utxo: JoinResult<Option<Transaction>>,
        /// The transaction output this input spends from. Should only be
        /// `Option::Some` for inputs which spend segwit outputs,
        /// including P2SH embedded ones.
        pub witness_utxo: JoinResult<Option<TxOut>>,
        /// A map from public keys to their corresponding signature as would be
        /// pushed to the stack from a scriptSig or witness for a non-taproot inputs.
        pub partial_sigs: JoinResult<BTreeMap<PublicKey, ecdsa::Signature>>,
        /// The sighash type to be used for this input. Signatures for this input
        /// must use the sighash type.
        pub sighash_type: JoinResult<Option<PsbtSighashType>>,
        /// The redeem script for this input.
        pub redeem_script: JoinResult<Option<ScriptBuf>>,
        /// The witness script for this input.
        pub witness_script: JoinResult<Option<ScriptBuf>>,
        /// A map from public keys needed to sign this input to their corresponding
        /// master key fingerprints and derivation paths.
        pub bip32_derivations: JoinResult<BTreeMap<secp256k1::PublicKey, KeySource>>,
        /// The finalized, fully-constructed scriptSig with signatures and any other
        /// scripts necessary for this input to pass validation.
        pub final_script_sig: JoinResult<Option<ScriptBuf>>,
        /// The finalized, fully-constructed scriptWitness with signatures and any
        /// other scripts necessary for this input to pass validation.
        pub final_script_witness: JoinResult<Option<Witness>>,
        /// TODO: Proof of reserves commitment
        /// RIPEMD160 hash to preimage map.
        pub ripemd160_preimages: JoinResult<BTreeMap<ripemd160::Hash, Vec<u8>>>,
        /// SHA256 hash to preimage map.
        pub sha256_preimages: JoinResult<BTreeMap<sha256::Hash, Vec<u8>>>,
        /// HSAH160 hash to preimage map.
        pub hash160_preimages: JoinResult<BTreeMap<hash160::Hash, Vec<u8>>>,
        /// HAS256 hash to preimage map.
        pub hash256_preimages: JoinResult<BTreeMap<sha256d::Hash, Vec<u8>>>,
        /// Serialized taproot signature with sighash type for key spend.
        pub tap_key_sig: JoinResult<Option<taproot::Signature>>,
        /// Map of `<xonlypubkey>|<leafhash>` with signature.
        pub tap_script_sigs:
            JoinResult<BTreeMap<(XOnlyPublicKey, TapLeafHash), taproot::Signature>>,
        /// Map of Control blocks to Script version pair.
        pub tap_scripts: JoinResult<BTreeMap<ControlBlock, (ScriptBuf, LeafVersion)>>,
        /// Map of tap root x only keys to origin info and leaf hashes contained in it.
        pub tap_key_origins: JoinResult<BTreeMap<XOnlyPublicKey, (Vec<TapLeafHash>, KeySource)>>,
        /// Taproot Internal key.
        pub tap_internal_key: JoinResult<Option<XOnlyPublicKey>>,
        /// Taproot Merkle root.
        pub tap_merkle_root: JoinResult<Option<TapNodeHash>>,
        /// Proprietary key-value pairs for this input.
        pub proprietaries: JoinResult<BTreeMap<raw::ProprietaryKey, Vec<u8>>>,
        /// Unknown key-value pairs for this input.
        pub unknowns: JoinResult<BTreeMap<raw::Key, Vec<u8>>>,
    }
}

pub use result::ResultInput;

// TODO impl Join, extract ok to CompoundResult trait
impl ResultInput {
    fn ok(self) -> Result<Input, Self> {
        match self {
            ResultInput {
                previous_txid: Ok(previous_txid),
                spent_output_index: Ok(spent_output_index),
                sequence: Ok(sequence),
                min_time: Ok(min_time),
                min_height: Ok(min_height),
                non_witness_utxo: Ok(non_witness_utxo),
                witness_utxo: Ok(witness_utxo),
                partial_sigs: Ok(partial_sigs),
                sighash_type: Ok(sighash_type),
                redeem_script: Ok(redeem_script),
                witness_script: Ok(witness_script),
                bip32_derivations: Ok(bip32_derivations),
                final_script_sig: Ok(final_script_sig),
                final_script_witness: Ok(final_script_witness),
                ripemd160_preimages: Ok(ripemd160_preimages),
                sha256_preimages: Ok(sha256_preimages),
                hash160_preimages: Ok(hash160_preimages),
                hash256_preimages: Ok(hash256_preimages),
                tap_key_sig: Ok(tap_key_sig),
                tap_script_sigs: Ok(tap_script_sigs),
                tap_scripts: Ok(tap_scripts),
                tap_key_origins: Ok(tap_key_origins),
                tap_internal_key: Ok(tap_internal_key),
                tap_merkle_root: Ok(tap_merkle_root),
                proprietaries: Ok(proprietaries),
                unknowns: Ok(unknowns),
            } => Ok(Input {
                previous_txid,
                spent_output_index,
                sequence,
                min_time,
                min_height,
                non_witness_utxo,
                witness_utxo,
                partial_sigs,
                sighash_type,
                redeem_script,
                witness_script,
                bip32_derivations,
                final_script_sig,
                final_script_witness,
                ripemd160_preimages,
                sha256_preimages,
                hash160_preimages,
                hash256_preimages,
                tap_key_sig,
                tap_script_sigs,
                tap_scripts,
                tap_key_origins,
                tap_internal_key,
                tap_merkle_root,
                proprietaries,
                unknowns,
            }),
            _ => Err(self),
        }
    }
}

#[test]
fn test_foo() {
    use std::collections::BTreeMap;
    use std::collections::HashMap;

    fn assert_impl_partial_join<T: PartialJoin>() {}
    assert_impl_partial_join::<Input>();
    assert_impl_partial_join::<Option<Input>>();
    assert_impl_partial_join::<BTreeMap<OutPoint, Input>>();
    assert_impl_partial_join::<HashMap<OutPoint, Input>>();

    fn assert_impl_hashkey<T: Clone + std::hash::Hash>() {}
    assert_impl_hashkey::<OutPoint>();

    // let b = Input { ..todo!() };
    // let a = Input { ..todo!() };
    // _ = PartialJoin::join(&a, &b);

    let oa: Option<Input> = None;
    let ob: Option<Input> = None;

    _ = PartialJoin::try_join(oa, ob);

    let ha = HashMap::<OutPoint, Input>::new();
    let hb = HashMap::<OutPoint, Input>::new();

    _ = PartialJoin::try_join(ha, hb);
}

#[test]
fn test_input_set() {
    InputSet::from_iter([Input {}]);
}
