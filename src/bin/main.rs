//------------------------------------------------------------------------------------------------//
// other modules

use clap;

//------------------------------------------------------------------------------------------------//
// own modules

//------------------------------------------------------------------------------------------------//

fn parse_cmdline<'a>() -> clap::ArgMatches<'a> {
    // arg: quiet
    let tmp = &[
        "Doesn't log 'info', but only 'warn' and 'error'.",
        "The env-variable 'RUST_LOG' has precedence.",
    ]
    .join("\n");
    let arg_quiet = clap::Arg::with_name("quiet")
        .short("q")
        .long("quiet")
        .help(tmp);

    // all
    clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .long_about(
            (&[
                "",
                "You can set up the logger by setting RUST_LOG, e.g. to",
                "    export RUST_LOG='warn,some_module=info,some_other_module=info'",
                "for getting 'warn's per default, but 'info' about the others.",
                "RUST_LOG is set up automatically, setting RUST_LOG to 'info'",
                "for relevant parts of the software, but consider the flag '--quiet'.",
                "",
                "In case you're using cargo, please use",
                "    cargo run --example",
                "for all supported example files",
            ]
            .join("\n"))
                .as_ref(),
        )
        .arg(arg_quiet)
        .get_matches()
}

fn setup_logging(quietly: bool) {
    let mut builder = env_logger::Builder::new();
    // minimum filter-level: `warn`
    builder.filter(None, log::LevelFilter::Warn);
    // if quiet logging: don't log `info` for the server and this repo
    if !quietly {
        builder.filter(Some(env!("CARGO_PKG_NAME")), log::LevelFilter::Info);
    }
    // overwrite default with environment-variables
    if let Ok(filters) = std::env::var("RUST_LOG") {
        builder.parse_filters(&filters);
    }
    if let Ok(write_style) = std::env::var("RUST_LOG_STYLE") {
        builder.parse_write_style(&write_style);
    }
    // init
    builder.init();
}

fn main() -> Result<(), ()> {
    let matches = parse_cmdline();
    setup_logging(matches.is_present("quiet"));

    if matches.args.len() == 0 {
        println!(
            "Execute '.../{} -h' (or 'cargo run -- -h') for more info.",
            env!("CARGO_PKG_NAME")
        );
    }

    Ok(())
}
