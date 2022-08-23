pub struct Profile {
    id: Id,
    name: Option<String>,
    midname: Option<String>,
    surname: Option<String>,
    email: Option<Email>,
    phone: Option<Phone>,
    photo: Option<Url>,
    address: Option<Address>,
}