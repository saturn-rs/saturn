use ratatui;
use saturn::{
    app::{App, Mode, Result},
    buffer::Buffer,
    config,
    events::{Event, EventBus, message::Message},
    plugin::Plugin,
    widgets::editor::Editor,
    widgets::prepare::Prepare,
};
use std::{cell::RefCell, rc::Rc};

/// A simple plugin ^-^
struct SimplePlugin {
    switches: RefCell<usize>,
}

/// Simple plugin implementation
impl SimplePlugin {
    /// Creates new simple plugin
    pub fn new() -> Self {
        Self { switches: 0.into() }
    }
}

/// Plugin implementation
impl Plugin for SimplePlugin {
    /// Initializes plugin
    fn init(self: Rc<Self>, event_bus: &mut EventBus) -> Message {
        event_bus.on(
            Event::PrepareOptionSwitch,
            Box::new(move |_| {
                let mut switches = self.switches.borrow_mut();
                *switches += 1;
                if *switches >= 10 {
                    Message::Quit
                } else {
                    Message::None
                }
            }),
        );
        Message::None
    }
}

fn main() -> Result<()> {
    let config = config::default_config();
    ratatui::run(|terminal| {
        App::new(Mode::Prepare(Prepare::new(&config.theme.prepare)), &config)
            .with_plugin(Rc::new(SimplePlugin::new()))
            .run(terminal)
    })
}
