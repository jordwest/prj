mod commands;
mod config;
mod discovery;

use config::Config;

fn main() {
    let config = Config::autoload().unwrap();
    // println!("{:?}", config);

    // commands::configure::configure().unwrap();
    // let root = traverse::Root::traverse(&config).unwrap();

    // println!("{:#?}", root);
    commands::cd::run(&config).unwrap();
}
