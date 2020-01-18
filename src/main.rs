mod commands;
mod config;
mod traverse;

use config::Config;

fn main() {
    println!("Hello, world!");

    let config = Config::autoload().unwrap();
    // println!("{:?}", config);

    // commands::configure::configure().unwrap();
    let root = traverse::Root::traverse(&config).unwrap();
    println!("{:#?}", root);
}
