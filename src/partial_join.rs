/// A result of a fallible Join
pub type JoinResult<V>
    // bounds on generic parameters in type aliases are not enforced, so even though
    // this makes sense we don't have it:
    // where
    //     V: PartialJoin + Clone,
    //     V::Error: Clone,
    = Result<V, <V as PartialJoin>::Error>;

/// Trait for a values implementing fallible join operation.
pub trait PartialJoin {
    /// The error type for when `join` fails.
    type Error;

    /// Merge two values of a given type into a new value of the same type
    /// incorporating the information of both inputs.
    ///
    /// This operation should be associative, commutative and idempotent.
    /// TODO consume self and other?
    fn try_join(self, other: Self) -> JoinResult<Self>
    where
        Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::*;

    impl<T> crate::collections::Absorb<T> for () {
        fn absorb(self, _other: T) -> Self {
            ()
        }
    }

    impl<T> crate::collections::Absorb<T> for core::convert::Infallible {
        fn absorb(self, _other: T) -> Self {
            self
        }
    }
    impl crate::join::Join for core::convert::Infallible {
        fn join(self, _other: Self) -> Self {
            self
        }
    }

    impl crate::join::Join for () {
        fn join(self, _other: Self) -> Self {
            ()
        }
    }

    impl crate::partial_join::PartialJoin for () {
        type Error = core::convert::Infallible;

        fn try_join(self, other: Self) -> crate::partial_join::JoinResult<Self> {
            Ok(crate::join::Join::join(self, other))
        }
    }

    #[test]
    fn test_trait_bounds() {
        fn assert_impl_partial_join_and_clone<T: PartialJoin + Clone>() {}
        assert_impl_partial_join_and_clone::<()>();
        assert_impl_partial_join_and_clone::<u8>();
        assert_impl_partial_join_and_clone::<Vec<u8>>();
        assert_impl_partial_join_and_clone::<crate::collections::vec::VecWrapper<u8>>();
        assert_impl_partial_join_and_clone::<Option<u8>>();
        assert_impl_partial_join_and_clone::<Option<Vec<u8>>>();
        // assert_impl_partial_join_and_clone::<crate::input::Input>();
        // assert_impl_partial_join_and_clone::<crate::output::Output>();
        // assert_impl_partial_join_and_clone::<Result<crate::input::Input, ()>>();
        // assert_impl_partial_join_and_clone::<JoinResult<crate::input::Input>>();
        // assert_impl_partial_join_and_clone::<JoinResult<crate::output::Output>>();

        // TODO move to join mod
        fn assert_impl_join_and_clone<T: crate::join::Join + Clone>() {}
        assert_impl_join_and_clone::<()>();
        assert_impl_join_and_clone::<Result<(), core::convert::Infallible>>();
        assert_impl_join_and_clone::<Result<u8, crate::values::ConflictingValues<u8>>>();
        // assert_impl_join_and_clone::<Result<crate::input::Input, ()>>();

        assert_eq!(PartialJoin::try_join((), ()), Ok(()));
        assert_eq!(crate::join::Join::join((), ()), ());
        assert_eq!(
            crate::join::Join::join(PartialJoin::try_join((), ()), Ok(())),
            Ok(())
        );
    }
}
