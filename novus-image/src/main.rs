use std::fs::File;
use std::path::Path;
use std::process::exit;

fn main() {
    println!("Looking for config file...");

    if !Path::new("config").exists() {
        println!("Couldn't find config file");
        exit(0);
    }
}
