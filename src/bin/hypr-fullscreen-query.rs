use std::io::{stderr, stdout, Write};

use hypr_fullscreen_listener::{args::Args, prelude::*, query};

/// Utility to output a text for a monitor full screen status.
/// The full screen state is retrieved from the active workspace in this monitor.
///
/// This executable prints the current status and finishes execution.
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
    let args = Args::from_arguments()?;
    query::monitor_fullscreen_status(&args)
}
