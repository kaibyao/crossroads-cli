use clap::{App, Arg, SubCommand};

pub fn cmd_run() -> App<'static, 'static> {
    SubCommand::with_name("run")
        .about("Runs the database API and optionally the web application server")
        .visible_alias("start")
        .arg(
            Arg::with_name("is_run_ui")
                .help("Also run the web application server")
                .long("ui")
                .short("u"),
        )
}
