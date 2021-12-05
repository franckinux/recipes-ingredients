use std::env;

// use crate::yaml_utils::load_file;

mod yaml_utils;


fn main() {
    let args: Vec<_> = env::args().collect();

    yaml_utils::load_file(&args[1]);
}
