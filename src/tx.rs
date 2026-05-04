pub use psbt_v2::v2::Psbt;

use crate::global::Global;
use crate::input::Input;
use crate::output::Output;

use crate::partial_join::JoinResult;
use crate::partial_join::PartialJoin;

mod sealed {
    use crate::collections::vec::VecWrapper;
    use crate::input::{Input, InputSet};
    use crate::output::{Output, OutputSet};
    use crate::partial_join::PartialJoin;

    pub trait Collection<T>: PartialJoin + FromIterator<T> + IntoIterator<Item = T> + Len {}

    impl Collection<Input> for VecWrapper<Input> {}

    impl Collection<Output> for VecWrapper<Output> {}

    pub trait Len {
        fn len(&self) -> usize;
    }

    impl<T> Len for VecWrapper<T> {
        fn len(&self) -> usize {
            Vec::len(self)
        }
    }

    impl Len for InputSet {
        fn len(&self) -> usize {
            InputSet::len(self)
        }
    }

    impl Len for OutputSet {
        fn len(&self) -> usize {
            OutputSet::len(self)
        }
    }

    #[test]
    fn test_collection() {
        fn assert_impl_partial_join<T: PartialJoin + Clone>() {}
        assert_impl_partial_join::<Input>();
        assert_impl_partial_join::<Output>();
        assert_impl_partial_join::<bitcoin::bip32::KeySource>();
        assert_impl_partial_join::<bitcoin::taproot::TapLeafHash>();
        //assert_impl_partial_join::<(bitcoin::taproot::TapLeafHash, bitcoin::bip32::KeySource)>();
        assert_impl_partial_join::<(
            Vec<bitcoin::taproot::TapLeafHash>,
            bitcoin::bip32::KeySource,
        )>();
        assert_impl_partial_join::<VecWrapper<Input>>();
        assert_impl_partial_join::<VecWrapper<Output>>();

        fn assert_impl_collection<V, T: Collection<V>>() {}
        assert_impl_collection::<Input, VecWrapper<Input>>();
        assert_impl_collection::<Output, VecWrapper<Output>>();
    }
}

pub trait Inputs: sealed::Collection<Input> {}
pub trait Outputs: sealed::Collection<Output> {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnorderedPsbt<I: Inputs, O: Outputs> {
    /// The global map.
    pub global: Global,
    /// The corresponding collection for each input in the unsigned transaction.
    pub inputs: I,
    /// The corresponding key-value map for each output in the unsigned transaction.
    pub outputs: O,
}

impl<I: Inputs, O: Outputs> PartialJoin for UnorderedPsbt<I, O> {
    type Error = (
        Result<Global, <Global as PartialJoin>::Error>,
        Result<I, I::Error>,
        Result<O, O::Error>,
    );

    fn try_join(self, other: Self) -> JoinResult<Self> {
        let global = self.global.try_join(other.global);
        let inputs = self.inputs.try_join(other.inputs);
        let outputs = self.outputs.try_join(other.outputs);

        match (global, inputs, outputs) {
            (Ok(global), Ok(inputs), Ok(outputs)) => Ok(Self {
                global,
                inputs,
                outputs,
            }),
            results => Err(results),
        }
    }
}

impl<I: Inputs, O: Outputs> UnorderedPsbt<I, O> {
    /// Infallible, lossy conversion from PSBT (forgets order). You probably
    /// want `crate::Constructor` instead.
    ///
    /// This constructor does not check that the PSBT is marked as unordered.
    pub fn from_psbt(psbt: Psbt) -> Self {
        Self {
            global: psbt.global,
            inputs: psbt.inputs.into_iter().collect(),
            outputs: psbt.outputs.into_iter().collect(),
        }
    }

    pub fn to_psbt(self) -> Psbt {
        Psbt {
            global: self.global,
            inputs: self.inputs.into_iter().collect(),
            outputs: self.outputs.into_iter().collect(),
        }
    }
}

#[test]
fn test_tx() {

}
