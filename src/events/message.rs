/// Imports
use crate::events::Event;

/// Defines a message an app can handle
#[derive(Clone)]
pub enum Message {
    /// Do nothing
    None,

    /// Quit app
    Quit,

    /// Fire an event
    Fire(Event),

    /// Many messages
    Many(Vec<Message>),
}
