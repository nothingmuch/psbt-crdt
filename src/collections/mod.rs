pub mod btreemap;
pub mod hashmap;
pub mod option;
pub mod result;
pub mod tuple;
pub mod vec;

pub trait Absorb<T> {
    fn absorb(self, other: T) -> Self;
}
