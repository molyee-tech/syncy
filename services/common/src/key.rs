pub struct Public(Box<str>);

pub struct Private(Box<str>);

pub struct Pair((Public, Private));