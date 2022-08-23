pub enum Login {
    Name(String),
    Phone(Phone),
    Email(Email),
    Pubkey(Pubkey),
}

pub struct Password {
    inner: String,
}

