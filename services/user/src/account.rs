use crate::user;

pub struct Id([u8; 32]);

pub struct Account {
    owner: user::Id,
}