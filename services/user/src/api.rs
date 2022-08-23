use crate::user::{self, User};
use async_trait::*;

#[async_trait]
pub trait Service {
    type Err;
    async fn create(&self, name: &str) -> Result<User, Self::Err>;
    async fn get(&self, id: user::Id) -> Result<User, Self::Err>;
}