use std::path::PathBuf;
use structopt::StructOpt;

use crate::yaml_utils::load_file;

mod yaml_utils;


#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Order files
    #[structopt(short, long, parse(from_os_str), default_value="data/prix-de-revient.yaml")]
    orders: Vec<PathBuf>,

    /// Html file
    #[structopt(short, long, parse(from_os_str))]
    html: Option<PathBuf>,

    /// Ingredients file
    #[structopt(short, long, parse(from_os_str), default_value="data/matieres-premieres/matieres-premieres.yaml")]
    ingredients: PathBuf,
}


fn main() {
    // let args: Vec<_> = env::args().collect();
    let opt = Opt::from_args();

    let ingredients = opt.ingredients.to_str().unwrap();

    println!("{:?}", ingredients);
    yaml_utils::load_file(ingredients);
}
