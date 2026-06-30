/// Imports
use crate::events::message::Message;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent},
    style::{Style, Stylize},
    widgets::{Block, List, ListState, StatefulWidget, Widget},
};
use std::{io, os::linux::raw::stat};

/// Defines prepare widget
pub struct Prepare {
    // List state
    state: ListState,
}

/// Prepare widget implementation
impl Prepare {
    /// Creates new prepare widget
    pub fn new() -> Self {
        // Preparing list state
        let mut state = ListState::default();
        state.select_first();

        // Done!
        Self { state }
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
impl Widget for &mut Prepare {
    /// Renders prepare menu
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        // A list of options
        let options = List::new(["Open File", "Exit"])
            .block(
                Block::bordered()
                    .border_style(Style::new().cyan())
                    .title("Welcome to 🪐 Saturn"),
            )
            .highlight_symbol("> ".green())
            .highlight_style(Style::new().italic())
            .scroll_padding(1)
            .direction(ratatui::widgets::ListDirection::TopToBottom);

        // Rendering options
        StatefulWidget::render(options, area, buf, &mut self.state);
    }
}
