pub struct Id(u32);

pub struct Network {
    name: String,
    desc: String,
}

pub struct Source {
    desc: String,
    network: net::Id,
    account: net::Address,
}
