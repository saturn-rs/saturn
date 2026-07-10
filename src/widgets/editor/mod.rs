/// Modules
mod bar;

/// Imports
use crate::app::{Error, Result};
use crate::events::Event;
use crate::io::IoError;
use crate::{
    buffer::Buffer,
    config::theme::EditTheme,
    events::message::Message,
    widgets::editor::bar::{Bar, StatusBar},
};
use ratatui::crossterm::event::{self};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Paragraph, Widget},
};

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
pub struct Editor<'t> {
    /// Buffer being edit
    buf: Buffer,

    /// Editor cursor
    cursor: Cursor,

    /// Editor bottom bar
    bar: Bar<'t>,

    /// Render offset
    offset: (usize, usize),

    // Theme reference
    theme: &'t EditTheme,
}

/// Implementation of editorstatus
impl<'t> Editor<'t> {
    /// Creates new editor
    pub fn new(buf: Buffer, status: &str, theme: &'t EditTheme) -> Self {
        Self {
            buf,
            cursor: Cursor::new(0, 0),
            bar: Bar::Status(StatusBar::new(&theme.status_bar_theme, status.to_string())),
            offset: (0, 0),
            theme,
        }
    }

    /// Initializes mode, returns init message
    pub fn init(&mut self) -> Message {
        Message::Fire(Event::EnterEditMode)
    }

    /// Sets bar to specified one
    pub fn set_bar(&mut self, bar: Bar<'t>) {
        self.bar = bar;
    }

    /// Renders text area
    fn render_text_area(&self, width: u16, height: u16) -> String {
        // Preparing render buffer
        let render_buf = String::new();

        // Getting rows by offset, width an height
        let _rows = self.buf.rows(self.offset, width as usize, height as usize);

        render_buf
    }

    /// Handles all the ratatui events
    pub fn handle_events(&mut self) -> Result<Message> {
        // Matching events read result
        match event::read() {
            // If ok, matching event
            Ok(_) => Ok(Message::None), // todo: handle events
            // Handling error
            Err(err) => Err(Error::IO(IoError::Unknown(err))),
        }
    }
}

/// Widget implementation
impl<'t> Widget for &mut Editor<'t> {
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
            .style(self.theme.code_widget_theme.style)
            .block(
                Block::bordered()
                    .style(self.theme.code_widget_theme.block_style)
                    .border_style(self.theme.code_widget_theme.block_border_style)
                    .title(self.buf.file_name()),
            )
            .render(layout[0], buf);

        // Rendering bar
        self.bar.render(layout[1], buf);
    }
}
