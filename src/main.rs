mod yaml_utils;

use std::collections::HashMap;
use std::path::PathBuf;
use structopt::StructOpt;

use yaml_utils::{General, Ingredient, Product, ReadStruct};



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


struct Proportion {
    recipe: String,
    quantity: i32,
    dough_weight: f64,
}


fn main() {
    let opt = Opt::from_args();

    // products
    let product: Product = Product::load_file("data/products/noisettes_raisins.yaml").unwrap();
    println!("{}", product.loss_rate);

    // recipes
    let recipes: HashMap<String, f64> = yaml_utils::load_file("data/recipes/mais.yaml").unwrap();
    println!("{}", recipes.get("farine_mais").unwrap());

    // ------------------------

    // general
    let general: General = General::load_file("data/general.yaml").unwrap();
    println!("{}", general.vat);

    // ingredients
    let ingredients_file = opt.ingredients.to_str().unwrap();
    let ingredient: HashMap<String, Ingredient> = Ingredient::load_file(ingredients_file).unwrap();
    // let farine = ingredient.get("farine_t65").unwrap();
    // println!("{}", farine.price);

    // orders
    let orders_file = opt.orders[0].to_str().unwrap();

    // where the result is collected
    // let mut proportions = HashMap::new();

    // loop over orders files
    for orders_pathbuf in opt.orders.iter() {
        let order_file = orders_pathbuf.to_str().unwrap();
        let orders: HashMap<String, i32> = yaml_utils::load_file(orders_file).unwrap();
        for (product, &quantity) in orders.iter() {
            println!("{} {}", product, quantity);

            if quantity == 0 {
                continue;
            }

            let product_file = format!("data/products/{}.yaml", product);
            println!("{}", product_file);
            let product_infos: Product = Product::load_file(product_file.as_str()).unwrap();

            let dough_weight = product_infos.dough_weight;
            let weight = dough_weight * f64::from(quantity);

            let proportion = Box::new(
                Proportion {
                    recipe: String::from(""),
                    quantity,
                    dough_weight,
                }
            );
            // proportions.insert(product, proportion);
        }
    }
}
