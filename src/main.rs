// #[cfg(test)]
// #[macro_use]
// extern crate pretty_assertions;

// #[macro_use]
// extern crate serde_derive;

extern crate clap;

// extern crate toml;
// use toml::Value;

mod commands;
mod config;

fn main() {
    commands::parse_cmd();

    // println!("{:?}", matches.subcommand());
}

// fn parse_cargo_toml() -> Value {}
