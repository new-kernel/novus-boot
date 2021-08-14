use std::fs::{read_to_string, File};
use std::path::Path;
use std::process::exit;

fn main() {
    println!("Looking for config file...");

    let mut config_file = String::new();

    if !Path::new("config.txt").exists() {
        println!("Couldn't find config file");
        exit(0);
    } else {
        config_file = read_to_string("config.txt").unwrap();
    }

    let kernel_file = config_file.replace("kernel=", "");
    if !Path::new(kernel_file.as_str()).exists() {
        println!("Couldn't find kernel file: {}", kernel_file);
    }
}
