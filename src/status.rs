use std::sync::Arc;

pub struct Status {
    pub is_fullscreen: bool,
    pub window_count: u16,
}

#[derive(Debug, Clone)]
pub struct Formatter {
    pub show_fullscreen_window_count: bool,
    pub fullscreen_text: Arc<str>,
    pub normal_mode_text: Arc<str>,
}

impl Formatter {
    #[must_use]
    pub fn format(&self, status: &Status) -> String {
        let text = if status.is_fullscreen {
            self.fullscreen_text.as_ref()
        } else {
            self.normal_mode_text.as_ref()
        };
        let win_count = status.window_count;
        let win_count =
            if status.is_fullscreen && self.show_fullscreen_window_count && win_count > 1 {
                format!(" +{}", win_count - 1)
            } else {
                String::new()
            };
        format!("{text}{win_count}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn format_status_fullscreen_with_count() {
        let formatter = Formatter {
            show_fullscreen_window_count: true,
            fullscreen_text: "Full Screen".into(),
            normal_mode_text: "".into(),
        };

        let status = Status {
            is_fullscreen: true,
            window_count: 2,
        };
        assert_eq!(formatter.format(&status), "Full Screen +1");
    }

    #[test]
    fn format_status_fullscreen_hide_count_for_one_window() {
        let formatter = Formatter {
            show_fullscreen_window_count: true,
            fullscreen_text: "Full Screen".into(),
            normal_mode_text: "".into(),
        };

        let status = Status {
            is_fullscreen: true,
            window_count: 1,
        };
        assert_eq!(formatter.format(&status), "Full Screen");
    }

    #[test]
    fn format_status_normal_mode() {
        let formatter = Formatter {
            show_fullscreen_window_count: true,
            fullscreen_text: "Full Screen".into(),
            normal_mode_text: "".into(),
        };

        let status = Status {
            is_fullscreen: false,
            window_count: 2,
        };
        assert_eq!(formatter.format(&status), "");
    }
}
