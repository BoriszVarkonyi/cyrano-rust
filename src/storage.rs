use std::fs::File;
use std::io::prelude::*;


//own imports
use crate::domain::Piste;

pub fn read_pistes() -> Result<Vec<Piste>, Box<dyn std::error::Error>>{
    let mut file = File::open("src/data/pistes.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let pistes: Vec<Piste> = serde_json::from_str(&contents)?;

    println!("{:?}", pistes);

    Ok(pistes)
}


