use std::io;

/// Imports
use crate::{buffer::Buffer, events::message::Message};
use ratatui::widgets::Widget;

/// Defines an editor widget
pub struct Editor {
    /// Buffer being edit
    buf: Buffer,
    // Editor cursor
    // cursor: Cursor,
}

/// Implementation of editor
impl Editor {
    /// Creates new editor
    pub fn new(buf: Buffer) -> Self {
        Self { buf }
    }

    /// Handles events
    pub fn handle_events(&mut self) -> io::Result<Message> {
        Ok(Message::None)
    }
}

/// Widget implementation
impl Widget for &mut Editor {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        todo!()
    }
}
