use std::io::{stderr, stdout, Write};

use hypr_fullscreen_status::{
    args::{self, formatter, MONITOR_ID, MONITOR_NAME},
    prelude::*,
    query,
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
    let mut args = args::query::matches();

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
                query::monitor_fullscreen_status_by_name,
            )
        },
        query::monitor_fullscreen_status_by_id,
    )?;

    let formatter = formatter(&mut args);

    Ok(formatter.format(&status))
}
