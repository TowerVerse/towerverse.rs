use crate::client::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Traveller {
    pub client: Client,
    pub id: String,
    pub name: String,
    pub email: String,
}
