use clap::{command, Arg, ArgMatches};
use super::{FULLSCREEN_TEXT, LISTEN, MONITOR_NAME, NORMAL_TEXT, SHOW_WINDOW_COUNT};

/// Parse the command line arguments into a `ArgMatches`.
///
/// Note that it is required to receive either a --monitor-id <ID> or --monitor-name <NAME>.
/// Optional arguments are: --fullscreen-text <TEXT> and --normal-text <TEXT>.
#[must_use]
pub fn matches() -> ArgMatches {
    command!()
        .arg(
            Arg::new(MONITOR_NAME)
                .long(MONITOR_NAME)
                .long_help("Monitor name")
                .value_name("NAME")
                .required(true),
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
        .arg(
            Arg::new(LISTEN)
                .long(LISTEN)
                .long_help("Keep listening to changes")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches()
}
