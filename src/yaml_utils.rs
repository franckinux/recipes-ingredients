use std::collections::HashMap;
// use std::io::prelude::*;
use std::fs::File;

use serde::Deserialize;
use serde::de::DeserializeOwned;


#[derive(Debug, PartialEq, Deserialize)]
pub struct Ingredient {
    pub price: f64,
    pub provider: String,
    pub conditioning: String,
}


impl Ingredient {
    pub fn load_file <T: DeserializeOwned> (filename: &str) -> Result<HashMap<String, T>, serde_yaml::Error> {
        let file = File::open(filename).expect(&format!("Unable to open file {}", filename));
        let deserialized: HashMap<String, T> = serde_yaml::from_reader(file)?;
        Ok(deserialized)
    }
}


#[derive(Debug, PartialEq, Deserialize)]
pub struct Product {
    pub recipe: String,
    pub loss_rate: f64,
    pub dough_weight: f64,
    pub bread_baked_weight: f64,
    pub sale_price_per_kg_tax_inclusive: f64,
}


impl Product {
    pub fn load_file <T: DeserializeOwned> (filename: &str) -> Result<T, serde_yaml::Error> {
        let file = File::open(filename).expect(&format!("Unable to open file {}", filename));
        let deserialized = serde_yaml::from_reader(file)?;
        Ok(deserialized)
    }
}


pub fn load_file <T: DeserializeOwned> (filename: &str) -> Result<HashMap<String, T>, serde_yaml::Error> {
    let file = File::open(filename).expect(&format!("Unable to open file {}", filename));
    let deserialized = serde_yaml::from_reader(file)?;
    Ok(deserialized)
}
