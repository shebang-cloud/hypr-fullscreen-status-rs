use hyprland::{
    data::{Monitors, Workspaces},
    shared::HyprData,
};

#[must_use]
pub fn query_monitor_status_by_name(name: impl AsRef<str>) -> Option<bool> {
    if let Ok(mut monitors) = Monitors::get() {
        if let Some(monitor) = monitors.find(|mon| mon.name == name.as_ref()) {
            return query_monitor_status(&monitor);
        }
    }
    None
}

#[must_use]
pub fn query_monitor_status_by_id(id: u8) -> Option<bool> {
    let id = i128::from(id);
    if let Ok(mut monitors) = Monitors::get() {
        if let Some(monitor) = monitors.find(|mon| mon.id == id) {
            return query_monitor_status(&monitor);
        }
    }
    None
}

fn query_monitor_status(monitor: &hyprland::data::Monitor) -> Option<bool> {
    let active_ws = &monitor.active_workspace;
    if let Ok(mut workspaces) = Workspaces::get() {
        if let Some(ws) = workspaces.find(|ws| ws.id == active_ws.id) {
            return Some(ws.fullscreen);
        }
    }
    None
}
