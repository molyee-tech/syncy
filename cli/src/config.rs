use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub hubs: Vec<String>,
    pub creds: Credentials,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Credentials {

}
