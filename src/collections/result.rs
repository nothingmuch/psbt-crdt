use crate::join::Join;
use crate::partial_join::{JoinResult, PartialJoin};

use super::Absorb;

impl<V, E> Join for JoinResult<V>
where
    V: PartialJoin<Error = E> + Clone,

    // The associated error must be `Join`, not just `PartialJoin` as well as
    // allow the value type to be transformed to an error type in so that
    // `join(Ok(a),Err(b) => Err(join(a, b))` can be infallible.
    E: Join + Absorb<V>,
{
    fn join(self, other: Self) -> Self {
        match (self, other) {
            (Ok(a), Ok(b)) => a.try_join(b),
            (Err(a), Err(b)) => Err(a.join(b)),
            (Err(a), Ok(b)) => Err(a.absorb(b.clone())),
            (Ok(a), Err(b)) => Err(b.absorb(a.clone())),
        }
    }
}

// // TODO remove
// // blanket impl does this
// impl<V, E> PartialJoin for Result<V, E>
// where
//     V: PartialJoin<Error = E> + Clone,
//     E: Join + Absorb<V>,
//     V::Error: Clone,
// {
//     type Error = std::convert::Infallible;

//     fn join(&self, other: &Self) -> JoinResult<Self> {
//         Ok(Join::join(self, other))
//     }
// }

#[test]
fn test_join_result() {
    assert_eq!(PartialJoin::try_join((), ()), Ok(()));
    assert_eq!(Join::join((), ()), ());
    assert_eq!(Join::join(PartialJoin::try_join((), ()), Ok(())), Ok(()));
}
