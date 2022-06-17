pub struct Id(u128);

pub struct Space {
    owner: account::Id,
    keys: Vec<Authority>,
}

pub struct Info {
    name: String,
    desc: String,
}
