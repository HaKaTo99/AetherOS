//! Event Queue System (Phase 12.2)
//! Thread-safe event handling for input and signals

use alloc::collections::VecDeque;
use spin::Mutex;

/// Generic event queue
pub struct EventQueue<T> {
    queue: Mutex<VecDeque<T>>,
}

impl<T> EventQueue<T> {
    /// Create a new event queue
    pub const fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
        }
    }

    /// Push an event to the queue
    pub fn push(&self, event: T) {
        self.queue.lock().push_back(event);
    }

    /// Pop an event from the queue
    pub fn pop(&self) -> Option<T> {
        self.queue.lock().pop_front()
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.queue.lock().is_empty()
    }

    /// Get queue length
    pub fn len(&self) -> usize {
        self.queue.lock().len()
    }
}

/// Input events
#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    KeyPress(u8),
    KeyRelease(u8),
    MouseMove(i16, i16),
    MouseButton(u8, bool),
}

/// Global input event queue
pub static INPUT_EVENTS: EventQueue<InputEvent> = EventQueue::new();

/// Event filter function type
pub type EventFilter<T> = fn(&T) -> bool;

/// Event router - routes events to specific queues
pub struct EventRouter<T> {
    routes: alloc::vec::Vec<(EventFilter<T>, EventQueue<T>)>,
}

impl<T: Clone> EventRouter<T> {
    pub fn new() -> Self {
        Self { routes: alloc::vec::Vec::new() }
    }

    /// Add a route: events matching the filter go to the target queue
    pub fn add_route(&mut self, filter: EventFilter<T>) -> &EventQueue<T> {
        self.routes.push((filter, EventQueue::new()));
        &self.routes.last().unwrap().1
    }

    /// Dispatch an event through all matching routes
    pub fn dispatch(&self, event: &T) {
        for (filter, queue) in &self.routes {
            if filter(event) {
                queue.push(event.clone());
            }
        }
    }
}

/// Multi-threaded event processor
pub struct EventProcessor<T: 'static> {
    source: &'static EventQueue<T>,
    handler: fn(T),
}

impl<T: 'static> EventProcessor<T> {
    pub fn new(source: &'static EventQueue<T>, handler: fn(T)) -> Self {
        Self { source, handler }
    }

    /// Process one event (call from scheduler tick)
    pub fn tick(&self) {
        if let Some(event) = self.source.pop() {
            (self.handler)(event);
        }
    }
}
