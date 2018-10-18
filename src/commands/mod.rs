use clap::{App, AppSettings};

mod new;
use self::new::{cmd_new, parse_new};

mod generate;
use self::generate::cmd_generate;

mod run;
use self::run::cmd_run;

/// Generates the command line tool interface
pub fn parse_cmd() {
    let matches = App::new("Crossroads CLI")
        .about("A CLI tool for managing Crossroads.")
        .bin_name("xr")
        .version("0.0.1")
        .author("Kai Yao <kai.b.yao@gmail.com>")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(cmd_new())
        .subcommand(cmd_generate())
        .subcommand(cmd_run())
        // .subcommand(cmd_import())
        // .subcommand(cmd_upgrade())
        .get_matches();

    match matches.subcommand() {
        ("new", Some(sub_cmd))       => {
            parse_new(sub_cmd)
        },
        ("generate", Some(_sub_cmd)) => {},
        ("run", Some(_sub_cmd))      => {},
        _                            => {
            println!("Invalid command");
        }
    }
}
