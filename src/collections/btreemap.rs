use std::collections::BTreeMap;

use crate::partial_join::{JoinResult, PartialJoin};

// TODO impl Join
//
// TODO tests

impl<K, V> PartialJoin for BTreeMap<K, V>
where
    K: Ord + Clone,
    V: PartialJoin + Clone,
{
    type Error = BTreeMap<K, JoinResult<V>>;

    fn try_join(self, other: Self) -> JoinResult<Self> {
        let mut new = BTreeMap::new();
        let mut all_ok = true;

        for (k, b) in self.into_iter().chain(other) {
            use std::collections::btree_map::Entry::*;

            match new.entry(k.clone()) {
                Occupied(mut entry) => {
                    let lub: &mut JoinResult<V> = entry.get_mut();

                    if let Ok(a) = lub {
                        *lub = a.clone().try_join(b); // FIXME no need to clone

                        if !lub.is_ok() {
                            all_ok = false;
                        }
                    } else {
                        panic!("should never happen: Vacant branch only inserts Ok() and there will be at most one key collision per pair of BTrees")
                    }
                }
                Vacant(entry) => {
                    entry.insert(Ok(b.clone()));
                }
            }
        }

        if all_ok {
            Ok(new
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        v.unwrap_or_else(|_| panic!("verified all nested results are Ok")),
                    )
                })
                .collect())
        } else {
            Err(new)
        }
    }
}
