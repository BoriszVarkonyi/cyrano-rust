use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Piste {
    pub name: String,
    pub ip_address : String,
    pub port : String
}

#[derive(Debug)]
pub struct Fencer {
    pub id : String,
    pub name : String,
    pub nation : String,
}