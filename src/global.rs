pub use psbt_v2::v2::Global;

use crate::partial_join::PartialJoin;

impl PartialJoin for Global {
    type Error = ResultGlobal;

    fn try_join(self, other: Self) -> Result<Self, Self::Error> {
        ResultGlobal {
            version: self.version.try_join(other.version),
            tx_version: self.tx_version.try_join(other.tx_version),
            fallback_lock_time: self.fallback_lock_time.try_join(other.fallback_lock_time),
            xpubs: self.xpubs.try_join(other.xpubs),
            //proprietaries: self.proprietaries.try_join(&other.proprietaries)?,
            proprietaries: PartialJoin::try_join(self.proprietaries, other.proprietaries),
            unknowns: self.unknowns.try_join(other.unknowns),
            tx_modifiable_flags: self.tx_modifiable_flags.try_join(other.tx_modifiable_flags),
            input_count: self.input_count.try_join(other.input_count),
            output_count: self.output_count.try_join(other.input_count),
        }
        .ok()
    }
}

mod result {
    pub use std::collections::BTreeMap;

    use bitcoin::bip32::{KeySource, Xpub};
    use bitcoin::locktime::absolute;
    use bitcoin::transaction;

    use psbt_v2::raw;
    use psbt_v2::Version;

    use crate::partial_join::JoinResult;

    pub struct ResultGlobal {
        /// The version number of this PSBT.
        pub version: JoinResult<Version>,

        /// The version number of the transaction being built.
        pub tx_version: JoinResult<transaction::Version>,

        /// The transaction locktime to use if no inputs specify a required locktime.
        pub fallback_lock_time: JoinResult<Option<absolute::LockTime>>,

        /// A bitfield for various transaction modification flags.
        pub tx_modifiable_flags: JoinResult<u8>,

        /// The number of inputs in this PSBT.
        pub input_count: JoinResult<usize>,

        /// The number of outputs in this PSBT.
        pub output_count: JoinResult<usize>,

        /// A map from xpub to the used key fingerprint and derivation path as defined by BIP 32.
        pub xpubs: JoinResult<BTreeMap<Xpub, KeySource>>,

        /// Global proprietary key-value pairs.
        pub proprietaries: JoinResult<BTreeMap<raw::ProprietaryKey, Vec<u8>>>,

        /// Unknown global key-value pairs.
        pub unknowns: JoinResult<BTreeMap<raw::Key, Vec<u8>>>,
    }
}

pub use result::ResultGlobal;

impl ResultGlobal {
    fn ok(self) -> Result<Global, Self> {
        match self {
            ResultGlobal {
                version: Ok(version),
                tx_version: Ok(tx_version),
                fallback_lock_time: Ok(fallback_lock_time),
                tx_modifiable_flags: Ok(tx_modifiable_flags),
                input_count: Ok(input_count),
                output_count: Ok(output_count),
                xpubs: Ok(xpubs),
                proprietaries: Ok(proprietaries),
                unknowns: Ok(unknowns),
            } => Ok(Global {
                version,
                tx_version,
                fallback_lock_time,
                tx_modifiable_flags,
                input_count,
                output_count,
                xpubs,
                proprietaries,
                unknowns,
            }),
            _ => Err(self),
        }
    }
}

// trait GlobalExt {
//     fn clear_inputs_ordered_flag(&mut self);
//     fn clear_outputs_ordered_flag(&mut self);
// }

// impl GlobalExt for Global {
//     fn clear_inputs_ordered_flag(&mut self) {
//         todo!("modify proprietary field")
//     }

//     fn clear_outputs_ordered_flag(&mut self) {
//         todo!("modify proprietary field")
//     }
// }

#[test]
fn test_global_fields() {
    // Global(todo!())
}
