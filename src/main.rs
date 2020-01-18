mod commands;
mod config;

fn main() {
    println!("Hello, world!");

    // let config = Config::autoload();
    // println!("{:?}", config);

    commands::configure::configure().unwrap();
}
