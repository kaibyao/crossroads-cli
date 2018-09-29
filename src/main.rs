// #[cfg(test)]
// #[macro_use]
// extern crate pretty_assertions;

extern crate clap;

// extern crate toml;
// use toml::Value;

mod commands;
mod constants;

fn main() {
    let matches = commands::parse_cmd();
    // let config_file = matches.value_of("config").unwrap_or(constants::DEFAULT_CONFIG_FILE_NAME);
}

// fn parse_cargo_toml() -> Value {}
