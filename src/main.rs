mod yaml_utils;

use std::collections::HashMap;
use std::path::PathBuf;
use structopt::StructOpt;

use yaml_utils::{Ingredient, Product};



#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Order files
    #[structopt(short, long, parse(from_os_str), default_value="data/orders/prix_revient.yaml")]
    orders: Vec<PathBuf>,

    /// Html file
    #[structopt(short, long, parse(from_os_str))]
    html: Option<PathBuf>,

    /// Ingredients file
    #[structopt(short, long, parse(from_os_str), default_value="data/ingredients/matieres_premieres.yaml")]
    ingredients: PathBuf,
}


fn main() {
    let opt = Opt::from_args();

    // general
    let general: HashMap<String, f64> = yaml_utils::load_file("data/general.yaml").unwrap();
    println!("{}", general.get("vat").unwrap());

    // ingredients
    let ingredients_file = opt.ingredients.to_str().unwrap();
    let ingredient: HashMap<String, Ingredient> = Ingredient::load_file(ingredients_file).unwrap();
    let farine = ingredient.get("farine_t65").unwrap();
    println!("{}", farine.price);

    // orders
    let orders_file = opt.orders[0].to_str().unwrap();
    let orders: HashMap<String, i32> = yaml_utils::load_file(orders_file).unwrap();
    println!("{}", orders.get("meteil").unwrap());

    // products
    let product: Product = Product::load_file("data/products/noisettes_raisins.yaml").unwrap();
    println!("{}", product.loss_rate);

    // recipes
    let recipes: HashMap<String, f64> = yaml_utils::load_file("data/recipes/mais.yaml").unwrap();
    println!("{}", recipes.get("farine_mais").unwrap());
}
