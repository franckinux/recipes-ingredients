extern crate yaml_rust;

use std::io::prelude::*;
use std::fs::File;
// use yaml_rust::yaml::{Hash, Yaml};
use yaml_rust::{yaml, YamlLoader};


fn print_indent(indent: usize) {
    for _ in 0..indent {
        print!("    ");
    }
}


fn dump_node(doc: &yaml::Yaml, indent: usize) {
    match *doc {
        yaml::Yaml::Array(ref v) => {
            for x in v {
                dump_node(x, indent + 1);
            }
        }
        yaml::Yaml::Hash(ref h) => {
            for (k, v) in h {
                print_indent(indent);
                println!("{:?}:", k);
                dump_node(v, indent + 1);
            }
        }
        _ => {
            print_indent(indent);
            println!("{:?}", doc);
        }
    }
}


pub fn load_file(file: &str) {
    let mut file = File::open(file).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&contents).unwrap();
    for doc in &docs {
        println!("---");
        dump_node(doc, 0);
    }
}

