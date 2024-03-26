use hyprland::event_listener::EventListenerMutable;

use crate::query;
use crate::prelude::*;
use crate::status::Formatter;

/// Listen to the fullscreen status of a monitor.
///
/// # Errors
/// Propagate any `HyprError`
pub fn listen_monitor_fullsecreen_status_by_name(name: String, formatter: Formatter) -> Result<()> {
    let mut event_listener = EventListenerMutable::new();

    event_listener.add_fullscreen_state_change_handler(move |_, state| {
        if state.active_monitor == name {
            if let Ok(status) = query::monitor_fullscreen_status_by_name(&name) {
                println!("{}", &formatter.format(&status));
            }
        }
    });

    event_listener.start_listener()?;

    Ok(())
}
