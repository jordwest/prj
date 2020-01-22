mod commands;
mod config;
mod discovery;

use clap::{App, SubCommand};
use config::Config;
use std::process::exit;

fn main() {
    let matches = App::new("prj")
        .version("1.0-alpha")
        .author("Jordan West")
        .about("Manage your local git projects")
        .subcommand(SubCommand::with_name("configure").about("create a configuration"))
        .subcommand(SubCommand::with_name("list").about("select a project"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("configure") {
        commands::configure::configure().unwrap();
    } else if let Some(_) = matches.subcommand_matches("list") {
        let config = Config::autoload().unwrap();

        if let Err(_) = commands::cd::run(&config) {
            exit(1);
        }
    } else {
        println!("{}", matches.usage());
        exit(1)
    }
}
