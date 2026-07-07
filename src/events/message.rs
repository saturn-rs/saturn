/// Imports
use std::collections::VecDeque;

/// Defines a message an app can handle
#[derive(Clone)]
pub enum Message {
    /// Do nothing
    None,

    /// Quit app
    Quit,

    /// Many messages
    Many(Vec<Message>),
}
