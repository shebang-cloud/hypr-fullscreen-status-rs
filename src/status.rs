pub struct Status {
    pub is_fullscreen: bool,
    pub window_count: u16,
}

pub struct Formatter {
    show_fullscreen_window_count: bool,
    fullscreen_text: String,
    normal_mode_text: String,
}

impl Formatter {
    #[must_use]
    pub const fn build(show_fullscreen_window_count: bool) -> FormatterBuilder {
        FormatterBuilder::new(show_fullscreen_window_count)
    }

    #[must_use]
    pub fn format(&self, status: &Status) -> String {
        let text = if status.is_fullscreen {
            &self.fullscreen_text
        } else {
            &self.normal_mode_text
        };
        let win_count = status.window_count;
        let win_count_text =
            if status.is_fullscreen && self.show_fullscreen_window_count && win_count > 1 {
                format!(" +{}", win_count - 1)
            } else {
                String::new()
            };
        format!("{text}{win_count_text}")
    }
}

pub struct FormatterBuilder {
    show_fullscreen_window_count: bool,
}
pub struct FormatterBuilderWithFullscreenText {
    show_fullscreen_window_count: bool,
    fullscreen_text: String,
}
pub struct FormatterBuilderWithNormalText {
    show_fullscreen_window_count: bool,
    normal_mode_text: String,
}

impl FormatterBuilder {
    #[must_use]
    pub const fn new(show_fullscreen_window_count: bool) -> Self {
        Self {
            show_fullscreen_window_count,
        }
    }

    #[must_use]
    pub fn fullscreen_text(self, text: impl Into<String>) -> FormatterBuilderWithFullscreenText {
        FormatterBuilderWithFullscreenText {
            show_fullscreen_window_count: self.show_fullscreen_window_count,
            fullscreen_text: text.into(),
        }
    }

    #[must_use]
    pub fn normal_text(self, text: impl Into<String>) -> FormatterBuilderWithNormalText {
        FormatterBuilderWithNormalText {
            show_fullscreen_window_count: self.show_fullscreen_window_count,
            normal_mode_text: text.into(),
        }
    }
}

impl FormatterBuilderWithFullscreenText {
    pub fn normal_mode_text(self, text: impl Into<String>) -> Formatter {
        Formatter {
            show_fullscreen_window_count: self.show_fullscreen_window_count,
            fullscreen_text: self.fullscreen_text,
            normal_mode_text: text.into(),
        }
    }
}

impl FormatterBuilderWithNormalText {
    pub fn fullscreen_text(self, text: impl Into<String>) -> Formatter {
        Formatter {
            show_fullscreen_window_count: self.show_fullscreen_window_count,
            fullscreen_text: text.into(),
            normal_mode_text: self.normal_mode_text,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_formatter() {
        let formatter = Formatter::build(true)
            .fullscreen_text("Full Screen")
            .normal_mode_text("");

        assert!(formatter.show_fullscreen_window_count);
        assert_eq!(formatter.fullscreen_text, "Full Screen".to_string());
        assert_eq!(formatter.normal_mode_text, String::new());
    }

    #[test]
    fn format_status() {
        let formatter = Formatter::build(true)
            .fullscreen_text("Full Screen")
            .normal_mode_text("");

        let status = Status {
            is_fullscreen: true,
            window_count: 2,
        };
        assert_eq!(formatter.format(&status), "Full Screen +1");
    }
}
