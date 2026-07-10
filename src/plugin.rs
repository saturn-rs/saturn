/// Imports
use crate::events::{EventBus, message::Message};
use std::rc::Rc;

/// Defines a plugin trait
pub trait Plugin {
    /// Should handle plugin init
    fn init(self: Rc<Self>, event_bus: &mut EventBus) -> Message;

    /// Should handle plugin deinit
    fn deinit(self: Rc<Self>, _: &mut EventBus) -> Message {
        Message::None
    }
}
