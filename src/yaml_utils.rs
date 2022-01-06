use std::collections::HashMap;
// use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use serde::Deserialize;
use serde::de::DeserializeOwned;


// ==========
// Reading a HashMap of struct, applies to RawMaterial only

#[derive(Debug, PartialEq, Deserialize)]
pub struct RawMaterial {
    pub price: f64,
    pub provider: String,
    pub conditioning: String,
}


impl RawMaterial {
    pub fn load_file <T: DeserializeOwned> (filename: &Path) -> Result<HashMap<String, T>, serde_yaml::Error> {
        let file = File::open(filename).expect(&format!("Unable to open file {}", filename.display()));
        let deserialized: HashMap<String, T> = serde_yaml::from_reader(file)?;
        Ok(deserialized)
    }
}


// ==========
// Reading a struct, applies to General and Product

pub trait ReadStruct {
    fn load_file <T: DeserializeOwned> (filename: &Path) -> Result<T, serde_yaml::Error> {
        let file = File::open(filename).expect(&format!("Unable to open file {}", filename.display()));
        let deserialized = serde_yaml::from_reader(file)?;
        Ok(deserialized)
    }
}


#[derive(Debug, PartialEq, Deserialize)]
pub struct General {
    pub vat: f64,
}


impl ReadStruct for General {
}


#[derive(Debug, PartialEq, Deserialize)]
pub struct Product {
    pub recipe: String,
    pub loss_rate: f64,
    pub dough_weight: f64,
    pub bread_baked_weight: f64,
    pub sale_price_per_kg_tax_inclusive: f64,
}


impl ReadStruct for Product {
}

// ==========
// This one is for recipes only

pub fn load_file <T: DeserializeOwned> (filename: &Path) -> Result<HashMap<String, T>, serde_yaml::Error> {
    let file = File::open(filename).expect(&format!("Unable to open file {}", filename.display()));
    let deserialized = serde_yaml::from_reader(file)?;
    Ok(deserialized)
}
