use clap::{App, Arg, SubCommand};

pub fn cmd_import() -> App<'static, 'static> {
    SubCommand::with_name("import")
        .about("Imports a database dump file into the configured database")
        .arg(
            Arg::with_name("is_overwrite_id")
                .help("Database host/address/ip")
                .long("overwrite-ids")
                .short("i"),
        )
        .arg(
            Arg::with_name("file")
                .help("path to database dump file")
                .long("file")
                .short("f"),
        )
}
