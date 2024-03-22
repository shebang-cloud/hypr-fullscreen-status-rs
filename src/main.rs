use clap::{command, value_parser, Arg, ArgGroup, ArgMatches};
use waybar_hypr_fullscreen_status::{
    prelude::*, query_fullscreen_status_by_monitor_id, query_fullscreen_status_by_monitor_name,
};

fn main() -> Result<()> {
    let mut args = arg_matches();

    let is_fullscreen = (args.remove_one::<u8>(MONITOR_ID).map_or_else(
        || {
            args.remove_one::<String>(MONITOR_NAME).map_or_else(
                || {
                    Err(Error::MissingArgument(format!(
                        "{MONITOR_ID} or {MONITOR_NAME}"
                    )))
                },
                query_fullscreen_status_by_monitor_name,
            )
        },
        query_fullscreen_status_by_monitor_id,
    ))?;

    let status: String = if is_fullscreen {
        args.remove_one::<String>(FULLSCREEN_TEXT)
            .ok_or_else(|| Error::MissingArgument(FULLSCREEN_TEXT.to_string()))?
    } else {
        args.remove_one::<String>(NORMAL_TEXT)
            .ok_or_else(|| Error::MissingArgument(NORMAL_TEXT.to_string()))?
    };

    println!("{status}");
    Ok(())
}

const MONITOR_ID: &str = "monitor-id";
const MONITOR_NAME: &str = "monitor-name";
const FULLSCREEN_TEXT: &str = "fullscreen-text";
const NORMAL_TEXT: &str = "normal-text";

fn arg_matches() -> ArgMatches {
    command!()
        .arg(
            Arg::new(MONITOR_ID)
                .long(MONITOR_ID)
                .long_help("Monitor ID")
                .value_name("ID")
                .value_parser(value_parser!(u8)),
        )
        .arg(
            Arg::new(MONITOR_NAME)
                .long(MONITOR_NAME)
                .long_help("Monitor name")
                .value_name("NAME"),
        )
        .group(
            ArgGroup::new("monitor")
                .required(true)
                .args([MONITOR_ID, MONITOR_NAME]),
        )
        .arg(
            Arg::new(FULLSCREEN_TEXT)
                .long(FULLSCREEN_TEXT)
                .long_help("Full screen text")
                .value_name("TEXT")
                .default_value("Full Screen"),
        )
        .arg(
            Arg::new(NORMAL_TEXT)
                .long(NORMAL_TEXT)
                .long_help("Normal mode text")
                .value_name("TEXT")
                .default_value(""),
        )
        .get_matches()
}
