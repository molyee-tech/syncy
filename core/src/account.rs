pub struct Id(u64);

pub struct Address(U256);

pub struct Account {
    owner: Id,
    address: Address,
}

pub struct Info {
    name: Box<str>,
    desc: Box<str>,
}
