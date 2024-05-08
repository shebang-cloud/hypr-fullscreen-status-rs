use hyprland::{
    data::{Monitors, Workspaces},
    shared::HyprData,
};

use crate::{args::Args, prelude::*};

/// Query a monitor fullscreen status.
/// The status is retrieved from the active workspace in this monitor.
///
/// # Errors
/// Propagate any `HyprError`
pub fn monitor_fullscreen_status(args: &Args) -> Result<String> {
    let name = &args.monitor_name;
    let monitors = Monitors::get()?;
    let monitor = monitors
        .iter()
        .find(|mon| &mon.name == name)
        .ok_or_else(|| {
            let names: Vec<String> = monitors.iter().map(|each| each.name.clone()).collect();
            Error::DataNotFoundIn(format!("monitor.name = {name}"), names)
        })?;

    match workspace_fullscreen_status(monitor.active_workspace.id) {
        Ok(status) => Ok(args.formatter.format(&status)),
        Err(err) => Err(err),
    }
}

/// Query the fullscreen status of a workspace.
///
/// # Errors
/// Propagate any `HyprError`
fn workspace_fullscreen_status(workspace_id: i32) -> Result<Status> {
    let workspaces = Workspaces::get()?;
    let workspace = workspaces
        .iter()
        .find(|ws| ws.id == workspace_id)
        .ok_or_else(|| Error::DataNotFound(format!("workspace.id = {workspace_id}")))?;

    Ok(Status {
        is_fullscreen: workspace.fullscreen,
        window_count: workspace.windows,
    })
}
