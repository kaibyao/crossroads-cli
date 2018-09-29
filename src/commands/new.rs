use clap::{App, Arg, SubCommand};

pub fn cmd_new() -> App<'static, 'static> {
    SubCommand::with_name("new")
        .about("Creates a new folder with a Crossroads configuration")
        .visible_alias("init")
        .arg(
            Arg::with_name("db_host")
                .help("Database host/address/ip")
                .long("dbhost")
                .short("h"),
        ).arg(
            Arg::with_name("db_port")
                .help("Database connection port")
                .long("dbport")
                .short("p"),
        ).arg(
            Arg::with_name("db_user")
                .help("Database user name/login")
                .long("dbuser")
                .short("u"),
        ).arg(
            Arg::with_name("db_pass")
                .help("Database user password")
                .long("dbpass")
                .short("w"),
        ).arg(
            Arg::with_name("db_name")
                .help("Database name")
                .long("dbname")
                .short("d"),
        ).subcommand(SubCommand::with_name("path").about("Where Crossroads files are generated"))
}
