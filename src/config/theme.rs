/// Imports
use ratatui::style::Style;
use serde::{Deserialize, Serialize};

/// Defines prepare mode options theme
#[derive(Serialize, Deserialize)]
pub struct OptionsTheme {
    /// List style
    pub list_style: Style,

    /// Open file option text
    pub open_file_text: String,

    /// Exit option text
    pub exit_text: String,
}

/// Defines prepare mode theme
#[derive(Serialize, Deserialize)]
pub struct PrepareTheme {
    /// Options theme
    pub options_theme: OptionsTheme,

    /// Higlight symbol
    pub highlight_symbol: String,

    /// Highlight symbol style
    pub highlight_symbol_style: Style,

    /// Highlight style
    pub highlight_style: Style,

    /// Block style
    pub block_style: Style,

    /// Block border style
    pub block_border_style: Style,

    /// Welcome title
    pub welcome_title: String,
}

/// Defines code widget theme
#[derive(Serialize, Deserialize)]
pub struct CodeWidgetTheme {
    /// Block style
    pub block_style: Style,

    /// Block border style
    pub block_border_style: Style,

    /// Code widget style
    pub style: Style,
}

/// Command bar theme
#[derive(Serialize, Deserialize)]
pub struct CommandBarTheme {
    /// Block style
    pub block_style: Style,

    /// Block border style
    pub block_border_style: Style,

    /// Command bar style
    pub style: Style,
}

/// Status bar theme
#[derive(Serialize, Deserialize)]
pub struct StatusBarTheme {
    /// Block style
    pub block_style: Style,

    /// Block border style
    pub block_border_style: Style,

    /// Status bar style
    pub style: Style,

    /// Emoji in front of status
    pub emoji: String,
}

/// Defines edit mode theme
#[derive(Serialize, Deserialize)]
pub struct EditTheme {
    /// Code widget theme
    pub code_widget_theme: CodeWidgetTheme,

    /// Status bar theme
    pub status_bar_theme: StatusBarTheme,

    /// Command bar theme
    pub command_bar_theme: CommandBarTheme,
}

/// Defines an app theme
#[derive(Serialize, Deserialize)]
pub struct Theme {
    /// Theme for prepare mode
    pub prepare: PrepareTheme,

    // Theme for edit mode
    pub edit: EditTheme,
}
