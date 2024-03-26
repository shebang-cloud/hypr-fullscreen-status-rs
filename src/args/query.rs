use clap::{command, value_parser, Arg, ArgGroup, ArgMatches};
use super::{FULLSCREEN_TEXT, MONITOR_ID, MONITOR_NAME, NORMAL_TEXT, SHOW_WINDOW_COUNT};

/// Parse the command line arguments into a `ArgMatches`.
///
/// Note that it is required to receive either a --monitor-id <ID> or --monitor-name <NAME>.
/// Optional arguments are: --fullscreen-text <TEXT> and --normal-text <TEXT>.
#[must_use]
pub fn matches() -> ArgMatches {
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
