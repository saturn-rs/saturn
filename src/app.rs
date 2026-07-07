/// Imports
use crate::{
    buffer::Buffer,
    config::Config,
    events::message::Message,
    io::{self, IoError},
    widgets::{editor::Editor, prepare::Prepare},
};
use ratatui::{DefaultTerminal, Frame};
use thiserror::Error;

/// Defines an application mode
pub enum Mode<'a> {
    /// Prepare to open file mode
    Prepare(Prepare<'a>),

    /// Edit mode
    Edit(Editor<'a>),
}

/// Defines an application error
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] io::IoError),
    #[error(transparent)]
    Ser(#[from] toml::ser::Error),
    #[error(transparent)]
    De(#[from] toml::de::Error),
}

/// Defines an application result
pub type Result<T> = core::result::Result<T, Error>;

/// Defines an application widget,
/// a container that holds title, editor and command bar.
pub struct App<'a> {
    /// Is app exited?
    exit: bool,

    /// App mode
    mode: Mode<'a>,

    /// App config
    config: &'a Config,
}

/// App implementation
impl<'a> App<'a> {
    /// Creates new application
    pub fn new(mode: Mode<'a>, config: &'a Config) -> Self {
        Self {
            exit: false,
            mode,
            config,
        }
    }

    /// Enters prepare mode and opens prepare widget
    pub fn prepare(&'a mut self) {
        self.mode = Mode::Prepare(Prepare::new(&self.config.theme.prepare));
    }

    /// Opens buffer for edit and enters edit mode
    /// with specified status in bar
    pub fn edit(&'a mut self, buf: Buffer, status: &str) {
        self.mode = Mode::Edit(Editor::new(buf, status, &self.config.theme.edit));
    }

    /// Runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        // While app is not exit
        while !self.exit {
            // Drawing widget depending on mode
            terminal
                .draw(|frame| self.draw(frame))
                .map_err(IoError::Unknown)
                .map_err(Error::IO)?;
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

    /// Handles all the ratatui events depending on mode
    pub fn handle_events(&mut self) -> Result<Message> {
        // Handling events by mode
        match &mut self.mode {
            Mode::Prepare(prepare) => prepare.handle_events(),
            Mode::Edit(editor) => editor.handle_events(),
        }
    }

    /// Handles a message
    fn handle_message(&mut self, message: Message) {
        // Handling message
        match message {
            // Doing nothing
            Message::None => {}

            // Quiting app
            Message::Quit => self.exit = true,

            // Handling many messages
            Message::Many(messages) => {
                for message in messages.into_iter().rev() {
                    self.handle_message(message);
                }
            }
        }
    }
}
