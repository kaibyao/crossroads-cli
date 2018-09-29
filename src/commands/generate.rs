use clap::{App, Arg, SubCommand};

pub fn cmd_generate() -> App<'static, 'static> {
    SubCommand::with_name("generate")
        .about("Creates the initial database tables")
        .help("Does not create tables used by front end web application by default")
        .arg(
            Arg::with_name("is_with_ui")
                .help("Also generate tables used by front end web application")
                .long("with-ui")
                .short("u"),
        ).arg(
            Arg::with_name("is_ui_only")
                .help("Only generate tables used by front end web application")
                .long("only-ui")
                .short("o"),
        ).arg(
            Arg::with_name("is_drop_db")
                .help("Drop the database & recreate before initializing tables")
                .long("drop")
                .short("d"),
        )
}
