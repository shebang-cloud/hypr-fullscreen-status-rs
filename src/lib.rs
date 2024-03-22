mod error;
pub mod prelude;

use hyprland::{
    data::{Monitors, Workspaces},
    shared::HyprData,
};
pub use prelude::*;

/// Query the fullscreen status of the active workspace of a monitor, by the workspace name.
///
/// # Errors
/// Propagate any `HyprError`
pub fn query_fullscreen_status_by_monitor_name(name: impl AsRef<str>) -> Result<bool> {
    let monitor = Monitors::get()?
        .find(|mon| mon.name == name.as_ref())
        .ok_or_else(|| Error::DataNotFound(format!("monitor.name = {}", name.as_ref())))?;

    query_monitor_status(monitor.active_workspace.id)
}

/// Query the fullscreen status of the active workspace of a monitor, by the workspace id.
///
/// # Errors
/// Propagate any `HyprError`
pub fn query_fullscreen_status_by_monitor_id(id: u8) -> Result<bool> {
    let id = i128::from(id);
    let monitor = Monitors::get()?
        .find(|mon| mon.id == id)
        .ok_or_else(|| Error::DataNotFound(format!("monitor.id = {id}")))?;

    query_monitor_status(monitor.active_workspace.id)
}

/// Query the fullscreen status of a workspace.
///
/// # Errors
/// Propagate any `HyprError`
fn query_monitor_status(workspace_id: i32) -> Result<bool> {
    let workspace = Workspaces::get()?
        .find(|ws| ws.id == workspace_id)
        .ok_or_else(|| Error::DataNotFound(format!("workspace.id = {workspace_id}")))?;

    Ok(workspace.fullscreen)
}