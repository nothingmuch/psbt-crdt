// use psbt_v2::v2::Constructor as Bip370Constructor;
// use psbt_v2::v2::Mod;
// use psbt_v2::v2::Modifiable;
// use psbt_v2::v2::Psbt;

// use crate::tx::UnorderedPsbt;
// use std::marker::PhantomData;

/// Marker for a `Constructor` with both inputs and outputs unordered, or for `Inputs` and `Outputs`.
pub enum Unordered {}
/// Marker for a `Constructor` with inputs unordered.
pub enum InputsOnlyUnordered {}
/// Marker for a `Constructor` with outputs unordered.
pub enum OutputsOnlyUnordered {}
// Marker for `Inputs` or `Outputs``
pub enum Ordered {}

mod sealed {
    pub trait Unord {}
    impl Unord for super::Unordered {}
    impl Unord for super::InputsOnlyUnordered {}
    impl Unord for super::OutputsOnlyUnordered {}
    impl Unord for super::Ordered {}
}

/// Marker for if either inputs or outputs are unordered, or both.
pub trait Unord: sealed::Unord + Sync + Send + Sized + Unpin {}

impl Unord for Unordered {}
impl Unord for InputsOnlyUnordered {}
impl Unord for OutputsOnlyUnordered {}
impl Unord for Ordered {}

// #[derive(Debug, Clone, PartialEq, Eq)]
//pub struct Constructor<M, I: Inputs, O: Outputs>(UnorderedPsbt<I, O>, PhantomData<M>);

// impl<M: Mod, O: Unord> Constructor<M, O> {
//     pub fn try_from_psbt(psbt: Psbt) -> Result<Self, ConstructorError> {
//
//     }
//
//     /// Just mark the inputs as ordered without sorting them. This does not
//     /// ensure a consistent ordering for all signers.
//     ///
//     /// You probably want `sort_inputs`
//     pub fn fix_input_order(self) {
//         todo!("set unordered inputs = false")
//     }
//
//     /// Just mark the outputs as ordered without sorting them. This does not
//     /// ensure a consistent ordering for all signers.
//     ///
//     /// You probably want `sort_outputs`
//     pub fn fix_output_order(self) {
//         todo!("set unordered outputs = false")
//     }
//
//     /// Just mark the inputs and outputs as ordered without sorting them. This does not
//     /// ensure a consistent ordering for all signers.
//     ///
//     /// You probably want `sort`
//     pub fn fix_order(self) {
//         todo!("set unordered = false")
//     }
//
//     pub fn sort(self) -> {
//         self.
//     }
//
//     pub fn sort_inputs(self) -> {
//         self.
//     }
//     pub fn sort_outputs(self) -> {
//         self.
//     }
//
//     /// Extract a PSBT for serialization.
//     pub fn into_psbt(self, )
//
// }

// impl Constructor<Modifiable, Unordered> {
//     pub fn new(psbt: Psbt) -> Result<Self, PsbtNotModifiableError> {
//         Ok(Bip370Constructor::<Modifiable>::new(psbt)?.into());
//     }
// }

// impl TryFrom<Bip370Constructor<Modifiable>> for Constructor<Modifiable, Unordered> {
//     fn try_from(constructor: Bip370Constructor<Modifiable>) -> Result{
//         constructor.psbt()
//     }
// }
