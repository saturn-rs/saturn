/// Imports
use crate::events::message::Message;

/// Defines an event id
#[derive(Clone, Copy, Debug, Default)]
pub struct EventId(usize);

/// Defines a hook in event system
pub type Hook = Box<dyn Fn(EventId) -> Message>;

/// Defines an event bus
pub struct EventBus {
    /// Event hooks map
    hooks: Vec<Vec<Hook>>,
}

/// Event bus implementation
impl EventBus {
    /// Creates new event bus
    pub fn new() -> Self {
        Self { hooks: Vec::new() }
    }

    /// Registers new event
    pub fn register(&mut self) -> EventId {
        self.hooks.push(Vec::new());
        EventId(self.hooks.len() - 1)
    }

    /// Fires an event, returns vec of messages
    pub fn fire(&mut self, id: EventId) -> Vec<Message> {
        let event_id = id.0;
        let mut messages = vec![];
        if event_id < self.hooks.len() {
            for hook in &self.hooks[event_id] {
                messages.push(hook(id));
            }
        } else {
            panic!("`{event_id}` is not a valid event id")
        }
        messages
    }

    /// Starts listening to specified event with passed hook
    pub fn listen(&mut self, id: EventId, hook: Hook) {
        let event_id = id.0;
        match self.hooks.get_mut(event_id) {
            Some(hooks) => hooks.push(hook),
            None => panic!("`{event_id}` is not a valid event id"),
        }
    }
}
