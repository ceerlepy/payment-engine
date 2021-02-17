use std::default::Default;
use serde::Serialize;

#[derive(Serialize)]
#[derive(Debug)]
pub struct ClientAccount{
    pub client: u16,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool
}

impl Default for ClientAccount {
    fn default() -> Self {
        ClientAccount { client: 0u16, available: 0.0, held: 0.0, total: 0.0, locked: false }
    }
}

impl ClientAccount {
    pub fn new(client: u16) -> ClientAccount{
        ClientAccount {client, .. ClientAccount::default()}
    }
}