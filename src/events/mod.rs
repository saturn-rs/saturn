/// Modules
pub mod message;

/// Imports
use crate::events::message::Message;
use std::collections::HashMap;

/// Defines an event
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Event {
    PrepareOptionSwitch,
    EnterPrepareMode,
    EnterEditMode,
}

/// Defines an event hook
pub type Hook = Box<dyn Fn(Event) -> Message>;

/// Defines an event bus
pub struct EventBus {
    /// Event hooks map
    hooks: HashMap<Event, Vec<Hook>>,
}

/// Event bus implementation
impl EventBus {
    /// Creates new event bus
    pub fn new() -> Self {
        Self {
            hooks: HashMap::new(),
        }
    }

    /// Fires an event, returns messages vector
    pub fn fire(&mut self, event: Event) -> Vec<Message> {
        // Getting hooks from map
        let hooks = self.hooks.entry(event).or_insert(Vec::new());

        // Iterating over hooks and firing them
        hooks.iter().map(|hook| hook(event)).collect()
    }

    /// Starts listening to specified event with passed hook
    pub fn on(&mut self, event: Event, hook: Hook) {
        // Getting hooks from map
        let hooks = self.hooks.entry(event).or_insert(Vec::new());

        // Inserting a new one
        hooks.push(hook);
    }
}
