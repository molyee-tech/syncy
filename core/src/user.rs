pub struct Id(u64);

pub struct User {
    name: String,
    keys: Vec<PubKey>,
}

pub struct Profile {
    icon: Url,
    page: Url,
}