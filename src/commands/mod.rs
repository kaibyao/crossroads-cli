use clap::{App, ArgMatches};

mod new;
use self::new::cmd_new;

mod generate;
use self::generate::cmd_generate;

mod run;
use self::run::cmd_run;

pub fn parse_cmd() -> ArgMatches<'static> {
    App::new("Crossroads CLI")
        .about("A CLI tool for managing Crossroads.")
        .bin_name("xr")
        .version("0.0.1")
        .author("Kai Yao <kai.b.yao@gmail.com>")
        // .arg(
        //     Arg::with_name("config")
        //         .help(&format!(
        //             "Sets a custom config file, defaults to {}",
        //             DEFAULT_CONFIG_FILE_NAME
        //         )).long("config")
        //         .short("c")
        //         .takes_value(true)
        //         .value_name("config_file"),
        // )
        .subcommand(cmd_new())
        .subcommand(cmd_generate())
        .subcommand(cmd_run())
        // .subcommand(cmd_import())
        // .subcommand(cmd_upgrade())
        .get_matches()
}
