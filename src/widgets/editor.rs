/// Imports
use crate::{buffer::Buffer, events::message::Message};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    text::Text,
    widgets::{Block, Paragraph, Widget},
};
use std::io;

/// Defines editor cursor
pub struct Cursor {
    /// Buffer row
    row: usize,

    /// Buffer column
    column: usize,
}

/// Cursor implementation
impl Cursor {
    /// Creates new cursor
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    /// Moves cursor to specified position
    pub fn move_to(&mut self, row: usize, column: usize) {
        (self.row, self.column) = (row, column);
    }

    /// Returns current row
    pub fn row(&self) -> usize {
        self.row
    }

    /// Returns current column
    pub fn column(&self) -> usize {
        self.column
    }
}

/// Defines an editor widget
pub struct Editor {
    /// Buffer being edit
    buf: Buffer,

    /// Editor cursor
    cursor: Cursor,

    /// Render offset
    offset: (usize, usize),
}

/// Implementation of editor
impl Editor {
    /// Creates new editor
    pub fn new(buf: Buffer) -> Self {
        Self {
            buf,
            cursor: Cursor::new(0, 0),
            offset: (0, 0),
        }
    }

    /// Renders text area
    fn render_text_area(&self, width: u16, height: u16) -> String {
        // Preparing render buffer
        let mut render_buf = String::new();

        // Getting rows by offset, width an height
        let rows = self.buf.rows(self.offset, width as usize, height as usize);

        render_buf
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
        // Preparing layout
        let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(85), Constraint::Percentage(15)],
        )
        .split(area);

        // Rendering code widget
        Paragraph::new(self.render_text_area(layout[0].width, layout[0].height))
            .block(
                Block::bordered()
                    .border_style(Style::new().cyan())
                    .title(self.buf.file_name()),
            )
            .render(layout[0], buf);

        // Rendering action bar
        Paragraph::new("File ready for edit")
            .block(Block::bordered().border_style(Style::new().cyan()))
            .centered()
            .render(layout[1], buf);
    }
}
