mod error;
pub mod prelude;
pub mod status;

use hyprland::{
    data::{Monitor, Monitors, Workspaces},
    shared::HyprData,
};

pub use prelude::*;
use status::Status;

/// Query a monitor fullscreen status by name.
/// The status is retrieved from the active workspace in this monitor.
///
/// # Errors
/// Propagate any `HyprError`
pub fn query_monitor_fullscreen_status_by_name(name: impl AsRef<str>) -> Result<Status> {
    let monitors: Vec<Monitor> = Monitors::get()?.collect();
    let monitor = monitors
        .iter()
        .find(|mon| mon.name == name.as_ref())
        .ok_or_else(|| {
            let names: Vec<String> = monitors.iter().map(|each| each.name.clone()).collect();
            Error::DataNotFoundIn(format!("monitor.name = {}", name.as_ref()), names)
        })?;

    query_workspace_fullscreen_status(monitor.active_workspace.id)
}

/// Query a monitor fullscreen status by id.
/// The status is retrieved from the active workspace in this monitor.
///
/// # Errors
/// Propagate any `HyprError`
pub fn query_monitor_fullscreen_status_by_id(id: u8) -> Result<Status> {
    let id = i128::from(id);
    let monitors: Vec<Monitor> = Monitors::get()?.collect();
    let monitor = monitors.iter().find(|mon| mon.id == id).ok_or_else(|| {
        let ids: Vec<String> = monitors.iter().map(|each| each.id.to_string()).collect();
        Error::DataNotFoundIn(format!("monitor.id = {id}"), ids)
    })?;

    query_workspace_fullscreen_status(monitor.active_workspace.id)
}

/// Query the fullscreen status of a workspace.
///
/// # Errors
/// Propagate any `HyprError`
fn query_workspace_fullscreen_status(workspace_id: i32) -> Result<Status> {
    let workspace = Workspaces::get()?
        .find(|ws| ws.id == workspace_id)
        .ok_or_else(|| Error::DataNotFound(format!("workspace.id = {workspace_id}")))?;

    Ok(Status {
        is_fullscreen: workspace.fullscreen,
        window_count: workspace.windows,
    })
}
