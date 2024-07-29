use std::io::{stderr, Write};

use hypr_fullscreen_listener::{
    args::Args,
    listener::{listen_monitor_fullsecreen_status, listen_window_open, listen_workspace_change},
    prelude::*,
};

/// Utility to output a text for a monitor full screen status.
/// The full screen state is retrieved from the active workspace in this monitor.
///
/// This executable blocks listening to status changes, always printing a new status line when it changes.
///
/// Use command line --help options to see required and optional command line arguments.
fn main() -> Result<()> {
    match Args::from_arguments() {
        Ok(args) => run_with_args(args),
        Err(err) => writeln!(stderr(), "{err}")?,
    }

    Ok(())
}

fn run_with_args(args: Args) {
    let args1 = args.clone();
    let args2 = args.clone();

    let desktop_thread = std::thread::spawn(move || {
        let _ignore = listen_workspace_change(args);
    });
    let fullscreen_thread = std::thread::spawn(move || {
        let _ignore = listen_monitor_fullsecreen_status(args1);
    });
    let windon_open_thread = std::thread::spawn(move || {
        let _ignore = listen_window_open(args2);
    });
    
    let _ignore = desktop_thread.join();
    let _ignore = fullscreen_thread.join();
    let _ignore = windon_open_thread.join();
}
