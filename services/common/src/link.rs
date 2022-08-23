pub enum Link {
    Url(Url),
    Ipfs(Ipfs),
}

pub struct Url(Box<str>);

pub struct Ipfs(Box<str>);