use std::sync::Arc;

use clap::{command, Arg, ArgMatches};

use crate::{prelude::*, status::Formatter};

const MONITOR_NAME: &str = "monitor-name";
const FULLSCREEN_TEXT: &str = "fullscreen-text";
const NORMAL_TEXT: &str = "normal-text";
const NO_SHOW_WINDOW_COUNT: &str = "no-show-window-count";

#[derive(Debug, Clone)]
pub struct Args {
    pub formatter: Formatter,
    pub monitor_name: Arc<str>,
}

impl Args {
    /// Build a `Args` object from command line arguments.
    ///
    /// # Errors
    /// Propagate any `IOError`
    pub fn from_arguments() -> Result<Self> {
        let mut args = matches();

        let Some(monitor_name) = args.remove_one::<String>(MONITOR_NAME) else {
            return Err(Error::MissingArgument(MONITOR_NAME.to_string()));
        };

        let formatter = build_formatter(&mut args);

        Ok(Self {
            formatter,
            monitor_name: monitor_name.into(),
        })
    }
}

fn build_formatter(args: &mut ArgMatches) -> Formatter {
    // These arguments have default value:
    let show_fullscreen_window_count = !args.get_flag(NO_SHOW_WINDOW_COUNT);

    let fullscreen_text: Arc<str> = (args
        .remove_one::<String>(FULLSCREEN_TEXT)
        .unwrap_or_else(|| "Full Screen".into()))
    .into();

    let normal_mode_text: Arc<str> =
        (args.remove_one::<String>(NORMAL_TEXT).unwrap_or_default()).into();

    Formatter {
        show_fullscreen_window_count,
        fullscreen_text,
        normal_mode_text,
    }
}

/// Parse the command line arguments into a `ArgMatches`.
fn matches() -> ArgMatches {
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
            Arg::new(NO_SHOW_WINDOW_COUNT)
                .long(NO_SHOW_WINDOW_COUNT)
                .long_help("Dont show window count in Full Screen mode")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches()
}
