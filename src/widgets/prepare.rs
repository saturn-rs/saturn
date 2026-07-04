/// Imports
use crate::{config::theme::PrepareTheme, events::message::Message};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    style::Styled,
    widgets::{Block, List, ListState, StatefulWidget, Widget},
};
use std::io;

/// Defines prepare widget
pub struct Prepare<'t> {
    // List state
    state: ListState,

    // Theme reference
    theme: &'t PrepareTheme,
}

/// Prepare widget implementation
impl<'t> Prepare<'t> {
    /// Creates new prepare widget
    pub fn new(theme: &'t PrepareTheme) -> Self {
        // Preparing list state
        let mut state = ListState::default();
        state.select_first();

        // Done!
        Self { state, theme }
    }

    /// Handles select event
    fn handle_select_event(&mut self) -> io::Result<Message> {
        // Matching state
        match self.state.selected() {
            // Second option
            Some(1) => Ok(Message::Quit),
            // Something other
            _ => Ok(Message::None),
        }
    }

    /// Handles key event
    fn handle_key_event(&mut self, event: KeyEvent) -> io::Result<Message> {
        // Matching key code
        match event.code {
            // Quit event
            KeyCode::Char('q') => Ok(Message::Quit),
            // Select previous event
            KeyCode::Up => {
                self.state.select_previous();
                Ok(Message::None)
            }
            // Select next event
            KeyCode::Down => {
                self.state.select_next();
                Ok(Message::None)
            }
            // Select key
            KeyCode::Enter => self.handle_select_event(),
            // Any other
            _ => Ok(Message::None),
        }
    }

    /// Handles events
    pub fn handle_events(&mut self) -> io::Result<Message> {
        // Matching events
        match event::read()? {
            // Handling key event
            Event::Key(event) => self.handle_key_event(event),
            // Ignoring any other
            _ => Ok(Message::None),
        }
    }
}

/// Widget implementation
impl<'t> Widget for &mut Prepare<'t> {
    /// Renders prepare menu
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        // A list of options
        let options = List::new([
            self.theme.options_theme.open_file_text.as_str(),
            self.theme.options_theme.exit_text.as_str(),
        ])
        .style(self.theme.options_theme.list_style)
        .block(
            Block::bordered()
                .style(self.theme.block_style)
                .border_style(self.theme.block_border_style)
                .title(self.theme.welcome_title.as_str()),
        )
        .highlight_symbol(
            self.theme
                .highlight_symbol
                .as_str()
                .set_style(self.theme.highlight_symbol_style),
        )
        .highlight_style(self.theme.highlight_style)
        .scroll_padding(1)
        .direction(ratatui::widgets::ListDirection::TopToBottom);

        // Rendering options
        StatefulWidget::render(options, area, buf, &mut self.state);
    }
}
