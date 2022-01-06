mod yaml_utils;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::PathBuf;
use structopt::StructOpt;

use yaml_utils::{General, RawMaterial, Product, ReadStruct};



#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Order files
    #[structopt(short, long, parse(from_os_str), default_value="prix_revient")]
    orders: Vec<PathBuf>,

    /// Html file
    #[structopt(short, long, parse(from_os_str))]
    html: Option<PathBuf>,

    /// RawMaterials file
    #[structopt(short, long, parse(from_os_str), default_value="matieres_premieres")]
    raw_materials: PathBuf,
}


#[derive(Debug)]
struct ProductEnd {
    recipe: Recipe,
    quantity: i32,
    dough_weight: f64,
}


type Recipe = HashMap<String, f64>;
type RawMaterials = HashMap<String, RawMaterial>;


fn follow_recipe(
    raw_materials: &RawMaterials,
    recipe_name: &str,
    recipe_quantity: f64
) -> (f64, Recipe) {
    match raw_materials.get(recipe_name) {
        None => {
            // the raw material is not found in the raw_materials hash map, so this is a recipe
            let mut recipe_pathbuf: PathBuf = [r"data", "recipes", recipe_name].iter().collect();
            recipe_pathbuf.set_extension("yaml");
            let recipe: Recipe = yaml_utils::load_file(recipe_pathbuf.as_path()).unwrap();

            let denominator: f64 = recipe.values().sum();

            let mut recipe_price = 0.0;
            let mut recipe_ingredients = Recipe::new();

            // loop over raw_materials of the recipe
            for (ingredient, ingredient_quantity) in recipe {
                let quantity = recipe_quantity * ingredient_quantity / denominator;
                // match recipe_ingredients.get_mut(ingredient.as_str()) {
                //     Some(ingredient) => {
                //         *ingredient += quantity;
                //     },
                //     None => {
                //         recipe_ingredients.insert(ingredient, quantity);
                //     },
                // };
                recipe_ingredients.insert(ingredient.clone(), quantity);

                let (ingredient_price, _) = follow_recipe(raw_materials, ingredient.as_str(), quantity);
                recipe_price += ingredient_price;
            };

            (recipe_price, recipe_ingredients)
        },
        Some(ingredient) => {
            println!("found ingredient {}", recipe_name);
            (ingredient.price, Recipe::new())
        },
    }
}


fn main() {
    let opt = Opt::from_args();

    // general
    let general_pathbuf = PathBuf::from("data/general.yaml");
    let general: General = General::load_file(general_pathbuf.as_path()).unwrap();

    // raw_material
    let raw_materials_file = opt.raw_materials.to_str().unwrap();
    let mut raw_materials_pathbuf: PathBuf = [r"data", "raw_materials", raw_materials_file].iter().collect();
    raw_materials_pathbuf.set_extension("yaml");
    let raw_material: RawMaterials = RawMaterial::load_file(raw_materials_pathbuf.as_path()).unwrap();

    // where the results are collected
    let mut products_end = HashMap::new();
    // let mut ingredients_end = HashMap::new();

    // loop over orders files
    for orders_pathbuf in opt.orders.iter() {
        let order_file = orders_pathbuf.to_str().unwrap();
        let mut order_pathbuf: PathBuf = [r"data", "orders", order_file].iter().collect();
        order_pathbuf.set_extension("yaml");
        let orders: HashMap<String, i32> = yaml_utils::load_file(order_pathbuf.as_path()).unwrap();

        // loop over products of the order
        for (order_product, &order_quantity) in orders.iter() {
            println!("from order file: product {},  quantity {}", order_product, order_quantity);

            if order_quantity == 0 {
                continue;
            }

            let mut product_pathbuf: PathBuf = [r"data", "products", order_product].iter().collect();
            product_pathbuf.set_extension("yaml");
            println!("{:?}", product_pathbuf.as_path());
            let product: Product = Product::load_file(product_pathbuf.as_path()).unwrap();

            let dough_weight = product.dough_weight;
            let loss_rate = product.loss_rate;
            let weight = dough_weight * f64::from(order_quantity);

            let (price, recipe) = follow_recipe(
                &raw_material,
                product.recipe.as_str(),
                weight * loss_rate
            );
            println!("price {}, recipe {:?}", price, recipe);

            products_end.insert(
                order_product.clone(),
                ProductEnd {
                    recipe,
                    quantity: order_quantity,
                    dough_weight,
                }
            );
        }
    }

    for v in products_end.iter() {
        println!("{:?}", v);
    }
}
