/// Modules
pub mod theme;

/// Imports
use crate::{
    config::theme::{
        CodeWidgetTheme, CommandBarTheme, EditTheme, OptionsTheme, PrepareTheme, StatusBarTheme,
        Theme,
    },
    io,
};
use miette::IntoDiagnostic;
use ratatui::style::Style;
use serde::{Deserialize, Serialize};

/// 🪐 Defines Saturn's configuration
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Theme configuration
    pub theme: Theme,
}

/// Returns default config
pub fn default_config() -> Config {
    Config {
        theme: Theme {
            prepare: PrepareTheme {
                options_theme: OptionsTheme {
                    list_style: Style::new(),
                    open_file_text: "Open File".into(),
                    exit_text: "Exit Saturn".into(),
                },
                highlight_symbol: "> ".to_owned(),
                highlight_symbol_style: Style::new().green(),
                highlight_style: Style::new().italic(),
                block_style: Style::new(),
                block_border_style: Style::new().cyan(),
                welcome_title: "Welcome to 🪐 Saturn".to_owned(),
            },
            edit: EditTheme {
                code_widget_theme: CodeWidgetTheme {
                    block_style: Style::new(),
                    block_border_style: Style::new().cyan(),
                    style: Style::new(),
                },
                command_bar_theme: CommandBarTheme {
                    block_style: Style::new(),
                    block_border_style: Style::new().cyan(),
                    style: Style::new(),
                },
                status_bar_theme: StatusBarTheme {
                    block_style: Style::new(),
                    block_border_style: Style::new().cyan(),
                    style: Style::new(),
                    emoji: "🛰️".to_owned(),
                },
            },
        },
    }
}

/// Reads config file if it exists,
/// and creates default one if it not
pub fn read_config() -> miette::Result<Config> {
    // Getting config path
    let path = io::config_path()?.join("saturn/config.toml");

    // If path exists
    if path.exists() {
        // Reading and parsing cconfig
        let text = io::read_file(path)?;
        let config = toml::from_str(&text).into_diagnostic()?;
        Ok(config)
    }
    // If not
    else {
        // Creating new default one
        io::create_dir(path.parent().unwrap())?;
        let config = default_config();
        let text = toml::to_string(&config).into_diagnostic()?;
        io::write_file(path, text)?;
        Ok(config)
    }
}
