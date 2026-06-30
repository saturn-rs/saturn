use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
};

use crate::{
    buffer::Buffer,
    events::message::Message,
    widgets::{editor::Editor, prepare::Prepare},
};

/// Defines an applicatiob mode
pub enum Mode {
    /// Prepare to open file mode
    Prepare(Prepare),

    /// Edit mode
    Edit(Editor),
}

/// Defines an application widget,
/// a container that holds title, editor and command bar.
pub struct App {
    /// Is app exited?
    exit: bool,

    /// App mode
    mode: Mode,
}

/// App implementation
impl App {
    /// Creates new application
    pub fn new(mode: Mode) -> Self {
        Self { exit: false, mode }
    }

    /// Opens prepare widget
    pub fn prepare(&mut self) {
        self.mode = Mode::Prepare(Prepare::new());
    }

    /// Opens buffer for edit and enters edit mode
    pub fn edit(&mut self, buf: Buffer) {
        self.mode = Mode::Edit(Editor::new(buf));
    }

    /// Runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        // While app is not exit
        while !self.exit {
            // Drawing widget depending on mode
            terminal.draw(|frame| self.draw(frame))?;
            // Handling events
            let message = self.handle_events()?;
            // Handling message
            self.handle_message(message);
        }
        Ok(())
    }

    /// Draws a frame depending on mode
    pub fn draw(&mut self, frame: &mut Frame<'_>) {
        // Rendering widget by mode
        match &mut self.mode {
            Mode::Prepare(prepare) => frame.render_widget(prepare, frame.area()),
            Mode::Edit(editor) => frame.render_widget(editor, frame.area()),
        }
    }

    /// Handles all the app events depending on mode
    pub fn handle_events(&mut self) -> io::Result<Message> {
        // Handling events by mode
        match &mut self.mode {
            Mode::Prepare(prepare) => prepare.handle_events(),
            Mode::Edit(editor) => editor.handle_events(),
        }
    }

    /// Handles a message
    fn handle_message(&mut self, message: Message) {
        // Matching a message
        match message {
            // Doing nothing
            Message::None => {}
            // Setting `exit` to true
            Message::Quit => self.exit = true,
        }
    }
}
