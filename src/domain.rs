use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Piste {
    pub name: String,
    pub ip_address : String,
    pub port : String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Match {
    pub round : i8,
    pub match_no : i8,
    pub match_order : String,
    pub fencer_1: String,
    pub fencer_1_score : i8,
    pub fencer_2: String,
    pub fencer_2_score: i8
}