extern crate toml;

use std::fs::{canonicalize, write, DirBuilder};
use clap::{App, Arg, ArgMatches, SubCommand};
use config::{Config, CONFIG_FILE_NAME};

/// Generates the Clap config for subcommand `new`.
pub fn cmd_new() -> App<'static, 'static> {
    SubCommand::with_name("new")
        .about("Creates a new folder with a Crossroads configuration")
        .visible_alias("init")
        .arg(
            Arg::with_name("path")
                .required(true)
                .help("Path where XR files are generated"),
        )
}

/// Parses the results of a `new` subcommand
pub fn parse_new(matches: &ArgMatches) {
    match matches.value_of("path") {
        Some(path) => create_project(path),
        None => panic!("Something wrong happened with command `new`: 1"),
    }
}

// Wrapper around create_project_folder(path: &str) and create_project_files(path: &str)
fn create_project(path: &str) {
    if create_project_folder(path) == () && create_project_files(path) == () {
        println!("Success: Created project folder at `{}`", path);
    }
}

fn create_project_folder(path: &str) -> () {
    DirBuilder::new().recursive(true).create(path).unwrap()
}

fn create_project_files(path: &str) -> () {
    // set up file path
    let mut file_path = canonicalize(path).unwrap();
    file_path.push(CONFIG_FILE_NAME);

    // set up file contents
    let conf = generate_default_config();
    let conf_toml = toml::to_string(&conf).unwrap();

    // write contents to file
    write(file_path, conf_toml).unwrap();
}

fn generate_default_config() -> Config {
    Config {
        db_host: "postgresql://localhost".to_string(),
        db_port: 5432,
        db_user: "user".to_string(),
        db_pass: "password".to_string(),
        db_name: "crossroads".to_string(),
    }
}
