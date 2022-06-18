pub struct Id(u128);

pub struct Unique<H: Hash> {
    hash: H,
    meta: Url,
    data: Url,
}
