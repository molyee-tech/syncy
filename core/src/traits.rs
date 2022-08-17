
use core::hash::Hasher;

pub struct Hash<const N: usize>([u8; N]);

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