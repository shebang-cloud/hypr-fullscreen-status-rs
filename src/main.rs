use std::io::{stdout, Write};

use clap::{command, value_parser, Arg, ArgGroup, ArgMatches};
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
    let mut args = arg_matches();

    let status = (args.remove_one::<u8>(MONITOR_ID).map_or_else(
        || {
            args.remove_one::<String>(MONITOR_NAME).map_or_else(
                || {
                    Err(Error::MissingArgument(format!(
                        "{MONITOR_ID} or {MONITOR_NAME}"
                    )))
                },
                query_monitor_fullscreen_status_by_name,
            )
        },
        query_monitor_fullscreen_status_by_id,
    ))?;

    let show_fullscreen_window_count = args.get_flag(SHOW_WINDOW_COUNT);
    let fullscreen_text: String = args
        .remove_one::<String>(FULLSCREEN_TEXT)
        .ok_or_else(|| Error::MissingArgument(FULLSCREEN_TEXT.to_string()))?;
    let normal_mode_text: String = args
        .remove_one::<String>(NORMAL_TEXT)
        .ok_or_else(|| Error::MissingArgument(NORMAL_TEXT.to_string()))?;

    let formatter = Formatter {
        show_fullscreen_window_count,
        fullscreen_text,
        normal_mode_text,
    };

    writeln!(stdout(), "{}", formatter.format(&status))?;
    Ok(())
}

const MONITOR_ID: &str = "monitor-id";
const MONITOR_NAME: &str = "monitor-name";
const FULLSCREEN_TEXT: &str = "fullscreen-text";
const NORMAL_TEXT: &str = "normal-text";
const SHOW_WINDOW_COUNT: &str = "show-window-count";

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
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches()
}
