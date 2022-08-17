use core::hash::Hasher;
use crate::hash::Hash;

trait Unique<H: Hasher, const N: usize> {
    fn token(&self) -> Hash<N>;
}

trait Amount<Value> {
    fn amount(&self) -> Value;
}

trait Meta {
    fn meta(&self) -> &[u8];
}

trait Asset {
    fn payload(&self) -> &[u8];
}

trait Owned<U> {
    fn owner(&self) -> U;
}