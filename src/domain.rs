use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Piste {
    pub name: String,
    pub ip_address : String,
    pub port : String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Match {
    pub round : String,
    pub match_no : String,
    pub match_order : String,
    pub fencer_1: String,
    pub fencer_2: String
}