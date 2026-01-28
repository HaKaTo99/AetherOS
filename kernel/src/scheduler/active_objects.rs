//! Active Objects Scheduler - Symbian DNA
//! Cooperative multitasking with message passing

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

use crate::arch::context::CpuContext;

pub struct ActiveObject {
    id: u32,
    process_id: u32, // [NEW] Link to PCB
    priority: u8,
    state: ObjectState,
    pub context: CpuContext, // [NEW] Task Context
    quantum: u32,        // Max time slice
    ticks_remaining: u32,// Current time slice remaining
    mailbox: [Message; MAX_MESSAGES],
    mailbox_head: usize,
    mailbox_tail: usize,
}

impl ActiveObject {
    pub const fn new(id: u32, priority: u8, process_id: u32) -> Self {
        Self {
            id,
            process_id,
            priority,
            state: ObjectState::Idle,
            context: CpuContext::empty(),
            quantum: 10,
            ticks_remaining: 10,
            mailbox: [Message::empty(); MAX_MESSAGES],
            mailbox_head: 0,
            mailbox_tail: 0,
        }
    }

    /// Create task with allocated stack and guard page
    pub fn new_with_stack(id: u32, priority: u8, process_id: u32, entry_point: u64) -> Self {
        const STACK_SIZE: usize = 64 * 1024; // 64KB
        
        unsafe {
            // Allocate stack from SMME (stack + guard page)
            use crate::SMME;
            let smme = &mut *core::ptr::addr_of_mut!(SMME);
            
            let stack_base = match smme.allocate(STACK_SIZE + 4096) {
                Ok(addr) => addr,
                Err(_) => {
                    // Fallback: use static stack area
                    0x100000 + (id as usize * (STACK_SIZE + 4096))
                }
            };
            
            // Setup stack guard page
            #[cfg(target_arch = "aarch64")]
            {
                use crate::memory::mmu::Mmu;
                Mmu::setup_stack_guard(stack_base, STACK_SIZE);
            }
            
            // Initialize context
            let sp = (stack_base + STACK_SIZE - 16) as u64; // 16-byte aligned
            let mut context = CpuContext::empty();
            context.sp = sp;
            context.x30 = entry_point; // Link register (return address)
            
            Self {
                id,
                process_id,
                priority,
                state: ObjectState::Ready,
                context,
                quantum: 10,
                ticks_remaining: 10,
                mailbox: [Message::empty(); MAX_MESSAGES],
                mailbox_head: 0,
                mailbox_tail: 0,
            }
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
        // Default process_id = 0 (Kernel) for now
        self.objects[self.object_count] = Some(ActiveObject::new(id, priority, 0));
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

    /// Tick the scheduler - called by hardware timer
    pub fn tick(&mut self) {
        // In a real OS, this would decrement the quantum of the running task
        // For simulation, we decrement active object's time
        let idx = self.current_object.load(Ordering::Relaxed) as usize;
        if let Some(Some(obj)) = self.objects.get_mut(idx) {
            if obj.state == ObjectState::Running {
                if obj.ticks_remaining > 0 {
                    obj.ticks_remaining -= 1;
                } else {
                    // Preempt!
                    obj.state = ObjectState::Ready;
                    obj.ticks_remaining = obj.quantum; // Reset quota
                }
            }
        }
    }

    /// Preemptive scheduling - Time slice based
    pub fn schedule(&mut self) {
        let mut scheduled = 0;
        
        // Round-robin with priority
        // In v1.1 we simply check if we can run
        for _ in 0..MAX_OBJECTS {
            let idx = self.current_object.load(Ordering::Relaxed) as usize;
            
            if let Some(Some(obj)) = self.objects.get_mut(idx) {
                if obj.state == ObjectState::Ready {
                    obj.state = ObjectState::Running;
                    
                    // Simulate processing time
                    if obj.ticks_remaining > 0 {
                        // Authorized to run
                        if let Some(msg) = obj.get_message() {
                            // Process...
                            scheduled += 1;
                        }
                    } else {
                        // Forced yield (should have been handled by tick, but double check)
                        obj.state = ObjectState::Ready;
                        obj.ticks_remaining = obj.quantum; // Reset
                        
                        // [ARCH] Context Switch Hook
                        // In v1.2 Real Mode:
                        // unsafe { 
                        //     let prev = current_obj.context;
                        //     let next = next_obj.context;
                        //     crate::arch::aarch64::__switch_context(&mut prev, &next); 
                        // }
                        
                         // Move to next object immediately
                        self.current_object.store(
                            ((idx + 1) % self.object_count) as u32,
                            Ordering::Relaxed
                        );
                        continue;
                    }

                    // Task state update logic
                    if obj.mailbox_head == obj.mailbox_tail {
                         obj.state = ObjectState::Idle;
                    } else if obj.ticks_remaining == 0 {
                         obj.state = ObjectState::Ready; // Yield
                    } else {
                         obj.state = ObjectState::Ready; // Cooperative yield for this loop
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
        // After schedule loop, one might remain ready if preemption logic loops
        // But in our current logic, schedule loop processes ONE message for each ready task and continues
        // or yields if time is up.
        // Assuming budget > 1 tick:
        assert!(stats.idle_objects + stats.ready_objects == 2);
    }

    #[test]
    fn test_preemption_logic() {
        let mut scheduler = ActiveObjectScheduler::new();
        let id = scheduler.create_object(10).unwrap();
        
        // Manually set state to Running for test
        // Since we can't easily force it via public API without messages, helper needed?
        // Or just trust internal access:
        if let Some(Some(obj)) = scheduler.objects.get_mut(id as usize) {
            obj.state = ObjectState::Running;
            obj.ticks_remaining = 2;
        }

        // Tick 1: Should still be running
        scheduler.tick();
        if let Some(Some(obj)) = scheduler.objects.get(id as usize) {
            assert_eq!(obj.state, ObjectState::Running);
            assert_eq!(obj.ticks_remaining, 1);
        }

        // Tick 2: Should preempt
        scheduler.tick();
        // Tick 3: (Logic says if ticks=0, preempt. Wait, logic was: if ticks>0 {ticks-=1} else {preempt})
        // Let's check logic:
        // if ticks_remaining > 0 { ticks_remaining -= 1 } else { Preempt }
        // So at 1: becomes 0. Still Running.
        // Next tick: ticks is 0. Else branch -> Preempt.
        
        scheduler.tick(); // Trigger preemption
        
        if let Some(Some(obj)) = scheduler.objects.get(id as usize) {
            assert_eq!(obj.state, ObjectState::Ready);
            assert_eq!(obj.ticks_remaining, 10); // Check reset to quantum
        }
    }
}
