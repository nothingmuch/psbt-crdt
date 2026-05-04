/// Trait for infallible join operation.
pub trait Join {
    /// Merge two values of a given type into a new value of the same type
    /// incorporating the information of both inputs.
    ///
    /// This operation should be associative, commutative and idempotent.
    fn join(self, other: Self) -> Self
    where
        Self: Sized;
}

// TODO no blanket PartialJoin for Join, instead add struct<T:Join> Partial(T) that impls PartialJoin
//
// impl<T> PartialJoin for T
// where
//     T: Join,
// {
//     type Error = std::convert::Infallible;

//     // TODO rename to try_join
//     fn join(&self, other: &Self) -> Result<Self, Self::Error> {
//         Ok(Join::join(self, other))
//     }
// }
