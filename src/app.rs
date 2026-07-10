/// Imports
use crate::{
    buffer::Buffer,
    config::Config,
    events::{EventBus, message::Message},
    io::{self, IoError},
    plugin::Plugin,
    widgets::{editor::Editor, prepare::Prepare},
};
use ratatui::{DefaultTerminal, Frame};
use std::rc::Rc;
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

    /// Active plugins
    plugins: Vec<Rc<dyn Plugin>>,

    /// Saturn's event bus
    pub event_bus: EventBus,
}

/// App implementation
impl<'a> App<'a> {
    /// Creates new application
    pub fn new(mut mode: Mode<'a>, config: &'a Config) -> Self {
        // Initializing mode
        let message = match &mut mode {
            Mode::Prepare(prepare) => prepare.init(),
            Mode::Edit(editor) => editor.init(),
        };

        // Preparing app
        let mut app = Self {
            exit: false,
            mode,
            config,
            plugins: Vec::new(),
            event_bus: EventBus::new(),
        };

        // Handling mode init message
        app.handle_message(message);

        // Done!
        app
    }

    /// Adds a new plugin to active plugins vec
    pub fn with_plugin(mut self, plugin: Rc<dyn Plugin>) -> Self {
        // Pushing plugin to active plugins vec
        self.plugins.push(plugin.clone());

        // Initializing plugin
        let message = plugin.init(&mut self.event_bus);
        self.handle_message(message);

        // Returning self
        self
    }

    /// Enters prepare mode and opens prepare widget
    pub fn prepare(&'a mut self) {
        // Preparing and initializing mode
        let mut prepare = Prepare::new(&self.config.theme.prepare);
        let message = prepare.init();

        // Handling init message
        self.handle_message(message);

        // Setting mode to prepare
        self.mode = Mode::Prepare(prepare);
    }

    /// Opens buffer for edit and enters edit mode
    /// with specified status in bar
    pub fn edit(&'a mut self, buf: Buffer, status: &str) {
        // Preparing and initializing mode
        let mut editor = Editor::new(buf, status, &self.config.theme.edit);
        let message = editor.init();

        // Handling init message
        self.handle_message(message);

        // Setting mode to edit
        self.mode = Mode::Edit(editor);
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

            // Firing an event
            Message::Fire(event) => {
                for message in self.event_bus.fire(event) {
                    self.handle_message(message);
                }
            }

            // Handling many messages
            Message::Many(messages) => {
                for message in messages.into_iter().rev() {
                    self.handle_message(message);
                }
            }
        }
    }
}

/// Drop implementation for App
impl<'a> Drop for App<'a> {
    // On drop
    fn drop(&mut self) {
        // Deinitializing plugins
        for plugin in std::mem::take(&mut self.plugins) {
            // Deinitializing plugin
            let message = plugin.deinit(&mut self.event_bus);
            self.handle_message(message);
        }
    }
}
