use std::io::{stderr, Write};

use hypr_fullscreen_status::{
    args::{self, formatter, MONITOR_NAME},
    listener::listen_monitor_fullsecreen_status_by_name,
    prelude::*,
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
    let mut args = args::listener::matches();

    let Some(monitor_name) = args.remove_one::<String>(MONITOR_NAME) else {
        let err = Error::MissingArgument(MONITOR_NAME.to_string());
        writeln!(stderr(), "{err}")?;
        return Err(err);
    };

    let formatter = formatter(&mut args);
    listen_monitor_fullsecreen_status_by_name(monitor_name, formatter)?;

    Ok(())
}
