pub mod listener;
pub mod query;

use clap::ArgMatches;
use crate::status::Formatter;

pub const MONITOR_ID: &str = "monitor-id";
pub const MONITOR_NAME: &str = "monitor-name";
pub const FULLSCREEN_TEXT: &str = "fullscreen-text";
pub const NORMAL_TEXT: &str = "normal-text";
pub const SHOW_WINDOW_COUNT: &str = "show-window-count";
pub const LISTEN: &str = "listen";

pub fn formatter(args: &mut ArgMatches) -> Formatter {
    // These arguments have default value:
    let show_fullscreen_window_count = args.get_flag(SHOW_WINDOW_COUNT);

    let fullscreen_text: String = args
        .remove_one::<String>(FULLSCREEN_TEXT)
        .unwrap_or_else(|| "Full Screen".to_string());

    let normal_mode_text: String = args
        .remove_one::<String>(NORMAL_TEXT)
        .unwrap_or_default();

    Formatter {
        show_fullscreen_window_count,
        fullscreen_text,
        normal_mode_text,
    }
}
