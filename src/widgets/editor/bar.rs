use crate::config::theme::StatusBarTheme;
use ratatui::widgets::{Block, Paragraph, Widget};

/// Defines status bar
pub struct StatusBar<'t> {
    /// Status bar theme
    theme: &'t StatusBarTheme,

    /// Current app status
    status: String,
}

/// Implementation
impl<'t> StatusBar<'t> {
    /// Creates new status bar
    pub fn new(theme: &'t StatusBarTheme, status: String) -> Self {
        Self { theme, status }
    }
}

/// Widget implementation
impl<'t> Widget for &StatusBar<'t> {
    /// Renders status bar
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Paragraph::new(format!("{} {}", self.theme.emoji, self.status))
            .style(self.theme.style)
            .block(
                Block::bordered()
                    .style(self.theme.block_style)
                    .border_style(self.theme.block_border_style),
            )
            .centered()
            .render(area, buf)
    }
}

/// Defines a bar widget
pub enum Bar<'t> {
    /// Command bar
    Command,

    /// Status bar
    Status(StatusBar<'t>),
}

/// Widget implementation
impl<'t> Widget for &Bar<'t> {
    /// Reneders bar depending on its kind
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        // Rendering widget by mode
        match self {
            Bar::Command => todo!("Not implemented yet :("),
            Bar::Status(bar) => bar.render(area, buf),
        }
    }
}
