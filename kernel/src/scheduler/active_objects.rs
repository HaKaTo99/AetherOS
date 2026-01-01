//! Active Objects Scheduler - Symbian DNA
//! Cooperative multitasking with message passing

#![no_std]

use core::sync::atomic::{AtomicU32, Ordering};

const MAX_OBJECTS: usize = 256;
const MAX_MESSAGES: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectState {
    Idle,
    Ready,
    Running,
    Waiting,
    Finished,
}

#[derive(Debug, Clone, Copy)]
pub struct Message {
    pub id: u32,
    pub data: u64,
}

impl Message {
    pub const fn empty() -> Self {
        Self { id: 0, data: 0 }
    }
}

pub struct ActiveObject {
    id: u32,
    priority: u8,
    state: ObjectState,
    mailbox: [Message; MAX_MESSAGES],
    mailbox_head: usize,
    mailbox_tail: usize,
}

impl ActiveObject {
    pub const fn new(id: u32, priority: u8) -> Self {
        Self {
            id,
            priority,
            state: ObjectState::Idle,
            mailbox: [Message::empty(); MAX_MESSAGES],
            mailbox_head: 0,
            mailbox_tail: 0,
        }
    }

    pub fn post_message(&mut self, msg: Message) -> Result<(), ()> {
        let next_tail = (self.mailbox_tail + 1) % MAX_MESSAGES;
        if next_tail == self.mailbox_head {
            return Err(()); // Mailbox full
        }
        
        self.mailbox[self.mailbox_tail] = msg;
        self.mailbox_tail = next_tail;
        
        if self.state == ObjectState::Idle {
            self.state = ObjectState::Ready;
        }
        
        Ok(())
    }

    pub fn get_message(&mut self) -> Option<Message> {
        if self.mailbox_head == self.mailbox_tail {
            return None;
        }
        
        let msg = self.mailbox[self.mailbox_head];
        self.mailbox_head = (self.mailbox_head + 1) % MAX_MESSAGES;
        msg
        Ok(())
    }

    pub fn post_message(&mut self, msg: Message) -> Result<(), ()> {
        let next_tail = (self.mailbox_tail + 1) % MAX_MESSAGES;
        if next_tail == self.mailbox_head {
            return Err(()); // Mailbox full
        }
        
        self.mailbox[self.mailbox_tail] = msg;
        self.mailbox_tail = next_tail;
        
        if self.state == ObjectState::Idle {
            self.state = ObjectState::Ready;
        }
        
        Ok(())
    }

    pub fn get_message(&mut self) -> Option<Message> {
        if self.mailbox_head == self.mailbox_tail {
            return None;
        }
        
        let msg = self.mailbox[self.mailbox_head];
        self.mailbox_head = (self.mailbox_head + 1) % MAX_MESSAGES;
        
        Some(msg)
    }
}

pub struct ActiveObjectScheduler {
    objects: [Option<ActiveObject>; MAX_OBJECTS],
    current_object: AtomicU32,
    object_count: usize,
}

impl ActiveObjectScheduler {
    pub const fn new() -> Self {
        const NONE: Option<ActiveObject> = None;
        Self {
            objects: [NONE; MAX_OBJECTS],
            current_object: AtomicU32::new(0),
            object_count: 0,
        }
    }

    pub fn create_object(&mut self, priority: u8) -> Result<u32, ()> {
        if self.object_count >= MAX_OBJECTS {
            return Err(());
        }
        
        let id = self.object_count as u32;
        self.objects[self.object_count] = Some(ActiveObject::new(id, priority));
        self.object_count += 1;
        
        Ok(id)
    }

    pub fn send_message(&mut self, to: u32, msg: Message) -> Result<(), ()> {
        if let Some(Some(obj)) = self.objects.get_mut(to as usize) {
            obj.post_message(msg)
        } else {
            Err(())
        }
    }

    /// Cooperative scheduling - Symbian style
    pub fn schedule(&mut self) {
        let mut scheduled = 0;
        
        // Round-robin with priority
        for _ in 0..MAX_OBJECTS {
            let idx = self.current_object.load(Ordering::Relaxed) as usize;
            
            if let Some(Some(obj)) = self.objects.get_mut(idx) {
                if obj.state == ObjectState::Ready {
                    obj.state = ObjectState::Running;
                    
                    // Process one message
                    if let Some(msg) = obj.get_message() {
                        // In real impl: Execute object's handler
                        // For now: Just mark as processed
                        scheduled += 1;
                    }
                    
                    // Check if more messages
                    if obj.mailbox_head == obj.mailbox_tail {
                        obj.state = ObjectState::Idle;
                    } else {
                        obj.state = ObjectState::Ready;
                    }
                    
                    break;
                }
            }
            
            // Move to next object
            self.current_object.store(
                ((idx + 1) % self.object_count) as u32,
                Ordering::Relaxed
            );
        }
    }

    pub fn stats(&self) -> SchedulerStats {
        let mut idle = 0;
        let mut ready = 0;
        let mut running = 0;
        
        for obj in self.objects.iter().flatten() {
            match obj.state {
                ObjectState::Idle => idle += 1,
                ObjectState::Ready => ready += 1,
                ObjectState::Running => running += 1,
                _ => {}
            }
        }
        
        SchedulerStats {
            total_objects: self.object_count,
            idle_objects: idle,
            ready_objects: ready,
            running_objects: running,
        }
    }
}

#[derive(Debug)]
pub struct SchedulerStats {
    pub total_objects: usize,
    pub idle_objects: usize,
    pub ready_objects: usize,
    pub running_objects: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_object() {
        let mut scheduler = ActiveObjectScheduler::new();
        let id = scheduler.create_object(10).unwrap();
        assert_eq!(id, 0);
    }

    #[test]
    fn test_message_passing() {
        let mut scheduler = ActiveObjectScheduler::new();
        let id = scheduler.create_object(10).unwrap();
        
        let msg = Message { id: 1, data: 42 };
        scheduler.send_message(id, msg).unwrap();
        
        let stats = scheduler.stats();
        assert_eq!(stats.ready_objects, 1);
    }

    #[test]
    fn test_scheduling() {
        let mut scheduler = ActiveObjectScheduler::new();
        let id1 = scheduler.create_object(10).unwrap();
        let id2 = scheduler.create_object(5).unwrap();
        
        scheduler.send_message(id1, Message { id: 1, data: 100 }).unwrap();
        scheduler.send_message(id2, Message { id: 2, data: 200 }).unwrap();
        
        scheduler.schedule();
        
        let stats = scheduler.stats();
        assert!(stats.idle_objects + stats.ready_objects == 2);
    }
}
