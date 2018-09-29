use clap::{App, Arg, SubCommand};

pub fn cmd_upgrade() -> App<'static, 'static> {
    SubCommand::with_name("run")
        .about("Upgrades the installation of a previous version of Crossroads to the current version")
}
