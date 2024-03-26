use clap::{command, value_parser, Arg, ArgGroup, ArgMatches};
use std::io::{stderr, stdout, Write};

use waybar_hypr_fullscreen_status::{
    prelude::*, query_monitor_fullscreen_status_by_id, query_monitor_fullscreen_status_by_name,
    status::Formatter,
};

/// Utility to output a text for a monitor full screen status.
/// The full screen state is retrieved from the active workspace in this monitor.
///
/// Note that it is required to receive either a --monitor-id <ID> or --monitor-name <NAME>.
/// Optional arguments are: --fullscreen-text <TEXT> and --normal-text <TEXT>.
///
/// Ex.: ./waybar-hypr-fullscreen-status --monitor-id 0
/// Ex.: ./waybar-hypr-fullscreen-status --monitor-name DP-1
fn main() -> Result<()> {
    match get_status_message() {
        Ok(msg) => writeln!(stdout(), "{msg}")?,
        Err(err) => writeln!(stderr(), "{err}")?,
    }

    Ok(())
}

/// Get the fullscreen state message retrieved
/// from the active workspace for the monitor specified in the arguments.
fn get_status_message() -> Result<String> {
    let mut args = arg_matches();

    let monitor_id: Option<u8> = args.remove_one(MONITOR_ID);
    let status = monitor_id.map_or_else(
        || {
            let monitor_name: Option<String> = args.remove_one(MONITOR_NAME);
            monitor_name.map_or_else(
                || {
                    Err(Error::MissingArgument(format!(
                        "{MONITOR_ID} or {MONITOR_NAME}"
                    )))
                },
                query_monitor_fullscreen_status_by_name,
            )
        },
        query_monitor_fullscreen_status_by_id,
    )?;

    // These arguments have default value:
    let show_fullscreen_window_count = args.get_flag(SHOW_WINDOW_COUNT);
    let fullscreen_text: String = args
        .remove_one::<String>(FULLSCREEN_TEXT)
        .ok_or_else(|| Error::MissingArgument(FULLSCREEN_TEXT.to_string()))?;
    let normal_mode_text: String = args
        .remove_one::<String>(NORMAL_TEXT)
        .ok_or_else(|| Error::MissingArgument(NORMAL_TEXT.to_string()))?;

    // Output formatter:
    let formatter = Formatter {
        show_fullscreen_window_count,
        fullscreen_text,
        normal_mode_text,
    };

    Ok(formatter.format(&status))
}

/// Parse the command line arguments into a `ArgMatches`.
///
/// Note that it is required to receive either a --monitor-id <ID> or --monitor-name <NAME>.
/// Optional arguments are: --fullscreen-text <TEXT> and --normal-text <TEXT>.
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
        .arg(
            Arg::new(SHOW_WINDOW_COUNT)
                .long(SHOW_WINDOW_COUNT)
                .long_help("Show window count in Full Screen mode")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches()
}

const MONITOR_ID: &str = "monitor-id";
const MONITOR_NAME: &str = "monitor-name";
const FULLSCREEN_TEXT: &str = "fullscreen-text";
const NORMAL_TEXT: &str = "normal-text";
const SHOW_WINDOW_COUNT: &str = "show-window-count";
