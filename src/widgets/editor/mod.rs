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
use ratatui::crossterm::event::{self, KeyCode, KeyEvent};
use ratatui::layout::Rect;
use ratatui::prelude;
use ratatui::style::Stylize;
use ratatui::text::{Line, Span, Text};
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

    /// Theme reference
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

    /// Adjusts offset to ensure cursor is in a viewport
    fn adjust_offset(&mut self, width: u16, height: u16) {
        let (width, height) = (width as usize, height as usize);

        // Cursor above viewport -> scroll up to reveal it
        if self.cursor.row < self.offset.0 {
            self.offset.0 = self.cursor.row;
        }
        // Cursor below viewport -> scroll down to reveal it
        else if self.cursor.row >= self.offset.0 + height {
            self.offset.0 = self.cursor.row - height + 1;
        }

        // Cursor left of viewport -> scroll left
        if self.cursor.column < self.offset.1 {
            self.offset.1 = self.cursor.column;
        }
        // Cursor right of viewport -> scroll right
        else if self.cursor.column >= self.offset.1 + width {
            self.offset.1 = self.cursor.column - width + 1;
        }
    }

    /// Moves cursor by delta
    fn move_cursor(&mut self, delta: (isize, isize)) {
        // Calculating row
        let max_row = self.buf.rows_amount().saturating_sub(1) as isize;
        let new_row = (self.cursor.row as isize + delta.0).clamp(0, max_row);
        self.cursor.row = new_row as usize;

        // Calculating column
        let max_column = self.buf.row(self.cursor.row).len() as isize;
        let new_column = (self.cursor.column as isize + delta.1).clamp(0, max_column);
        self.cursor.column = new_column as usize;
    }

    /// Renders text area
    fn render_text_area(&self, width: u16, height: u16) -> Text<'static> {
        self.buf
            .rows(self.offset, width as usize, height as usize)
            .iter()
            .map(|row| {
                // Calculating left padding for line number
                let left_padding = Span::raw(" ".repeat(
                    self.buf.rows_amount().to_string().len() - row.idx().to_string().len(),
                ));

                // Preparing line number and gap between line number and line text
                let line_number = row.idx().to_string().light_cyan();
                let gap = Span::raw(" ");

                // Preparing line text
                let line_text = Span::raw(row.text().to_string());

                // Building line
                Line::from(vec![left_padding, line_number, gap, line_text])
            })
            .collect::<Vec<_>>()
            .into()
    }

    /// Renders cursor in rect
    fn render_cursor(&self, area: Rect, buf: &mut prelude::Buffer) {
        // Calculating gutter
        let gutter_width = self.buf.rows_amount().to_string().len() + 1;
        let (row_offset, col_offset) = self.offset;

        // If cursor outside buffer
        if self.cursor.row < row_offset || self.cursor.column < col_offset {
            return;
        }

        // Calculating relative row and column
        let (rel_row, rel_col) = (
            self.cursor.row - row_offset,
            self.cursor.column - col_offset,
        );

        // +1 because of border
        let x = area.x + 1 + gutter_width as u16 + rel_col as u16;
        let y = area.y + 1 + rel_row as u16;

        // If cursor outside buffer
        if x >= area.x + area.width.saturating_sub(1) || y >= area.y + area.height.saturating_sub(1)
        {
            return;
        }

        // Updating cell
        if let Some(cell) = buf.cell_mut((x, y)) {
            cell.set_style(self.theme.code_widget_theme.cursor_style);
        }
    }

    /// Handles key event
    fn handle_key_event(&mut self, event: KeyEvent) -> Result<Message> {
        // Matching key code
        match event.code {
            // Quit event
            KeyCode::Esc => Ok(Message::Quit),
            // Cursor up event
            KeyCode::Up => {
                self.move_cursor((-1, 0));
                Ok(Message::Fire(Event::CursorMove(-1, 0)))
            }
            // Cursor down event
            KeyCode::Down => {
                self.move_cursor((1, 0));
                Ok(Message::Fire(Event::CursorMove(1, 0)))
            }
            // Cursor left event
            KeyCode::Left => {
                self.move_cursor((0, -1));
                Ok(Message::Fire(Event::CursorMove(0, -1)))
            }
            // Cursor right event
            KeyCode::Right => {
                self.move_cursor((0, 1));
                Ok(Message::Fire(Event::CursorMove(0, 1)))
            }
            // Cursor to start event
            KeyCode::Home => {
                // Calculating delta column
                let delta_column = -(self.cursor.column as isize);

                // Moving cursor
                self.move_cursor((0, delta_column));
                Ok(Message::Fire(Event::CursorMove(0, delta_column)))
            }
            // Cursor to end event
            KeyCode::End => {
                // Calculating delta column
                let row_len = self.buf.row(self.cursor.row).len();
                let delta_column = (row_len - self.cursor.column) as isize;

                // Moving cursor
                self.move_cursor((0, delta_column));
                Ok(Message::Fire(Event::CursorMove(0, delta_column)))
            }
            // Cursor to page up event
            KeyCode::PageUp => {
                // Calculating delta row
                let delta_row = -(self.cursor.row as isize);

                // Moving cursor
                self.move_cursor((delta_row, 0));
                Ok(Message::Fire(Event::CursorMove(delta_row, 0)))
            }
            // Cursor to page down event
            KeyCode::PageDown => {
                // Calculating delta row
                let delta_row = (self.buf.rows_amount() - self.cursor.row) as isize;

                // Moving cursor
                self.move_cursor((delta_row, 0));
                Ok(Message::Fire(Event::CursorMove(delta_row, 0)))
            }
            // Any other
            _ => Ok(Message::None),
        }
    }

    /// Handles all the ratatui events
    pub fn handle_events(&mut self) -> Result<Message> {
        // Matching events read result
        match event::read() {
            // If ok, matching event
            Ok(event) => match event {
                // Handling key event
                event::Event::Key(event) => self.handle_key_event(event),
                // Ignoring any other
                _ => Ok(Message::None),
            },
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

        // Adjusting offset
        self.adjust_offset(layout[0].width, layout[0].height);

        // Rendering cursor
        self.render_cursor(layout[0], buf);

        // Rendering bar
        self.bar.render(layout[1], buf);
    }
}
