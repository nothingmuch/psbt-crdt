use crate::join::Join;
use crate::partial_join::{JoinResult, PartialJoin};

impl<V> Join for Option<V>
where
    V: Join + Clone,
{
    fn join(self, other: Self) -> Self {
        match (self, other) {
            (None, None) => None,
            (None, x) | (x, None) => x.clone(),
            (Some(a), Some(b)) => Some(a.join(b)),
        }
    }
}

impl<V> PartialJoin for Option<V>
where
    V: PartialJoin + Clone,
{
    type Error = V::Error;

    fn try_join(self, other: Self) -> JoinResult<Self> {
        Ok(match (self, other) {
            (None, None) => None,
            (None, x) | (x, None) => x.clone(),
            (Some(a), Some(b)) => Some(a.try_join(b)?),
        })
    }
}

impl<T: crate::values::IdempotentValue> crate::collections::Absorb<Option<T>>
    for crate::values::ConflictingValues<T>
{
    fn absorb(self, other: Option<T>) -> Self {
        if let Some(value) = other {
            crate::collections::Absorb::<T>::absorb(self, value)
        } else {
            self.clone()
        }
    }
}

#[test]
fn test_join_option() {
    assert_eq!(Join::join(None::<()>, None), None);
    assert_eq!(Join::join(Some(()), None), Some(()));
    assert_eq!(Join::join(None, Some(())), Some(()));
    assert_eq!(Join::join(Some(()), Some(())), Some(()));

    let a = 1u8;

    assert_eq!(PartialJoin::try_join(None::<u8>, None), Ok(None));
    assert_eq!(PartialJoin::try_join(Some(a), None), Ok(Some(a)));
    assert_eq!(PartialJoin::try_join(None, Some(a)), Ok(Some(a)));
    assert_eq!(PartialJoin::try_join(Some(a), Some(a)), Ok(Some(a)));

    let b = 2u8;

    assert_eq!(
        PartialJoin::try_join(Some(a), Some(b)),
        Err(crate::values::ConflictingValues([1u8, 2].into()))
    );

    let ab = PartialJoin::try_join(Some(b), Some(a));
    let bb = PartialJoin::try_join(Some(b), Some(b));

    let conflict = crate::values::ConflictingValues([a, b].into());

    assert_eq!(ab, Err(conflict.clone()));
    assert_eq!(bb, Ok(Some(b)));
    assert_eq!(Join::join(ab.clone(), bb.clone()), Err(conflict),);
}
