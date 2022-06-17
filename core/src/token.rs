pub struct Id(u128);

pub struct Token {
    unique: Unique,
    source: source::Id,
    parent: Option<Id>,
}

pub struct Info {
    name: String,
    desc: String,
}
