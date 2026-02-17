//! Active Objects Scheduler - Symbian DNA
//! Priority-based preemptive multitasking with message passing

use core::sync::atomic::{AtomicU32, AtomicU64, AtomicBool, Ordering};

const MAX_OBJECTS: usize = 256;
const MAX_MESSAGES: usize = 16;
const PRIORITY_LEVELS: usize = 8;

/// Task states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectState {
    Idle,
    Ready,
    Running,
    Waiting,
    Blocked,
    Finished,
}

/// Message for inter-task communication
#[derive(Debug, Clone, Copy)]
pub struct Message {
    pub id: u32,
    pub data: u64,
    pub sender: u32,
    pub priority: u8,
}

impl Message {
    pub const fn empty() -> Self {
        Self { id: 0, data: 0, sender: 0, priority: 0 }
    }
}

use crate::arch::context::CpuContext;

/// Active Object (Task)
pub struct ActiveObject {
    pub id: u32,
    pub process_id: u32,
    pub priority: u8,
    pub base_priority: u8,      // Original priority (for inheritance)
    pub state: ObjectState,
    pub context: CpuContext,
    pub quantum: u32,           // Max time slice
    pub ticks_remaining: u32,   // Current time slice remaining
    pub cpu_affinity: u8,       // Which CPUs can run this task (bitmask)
    pub total_ticks: u64,       // Total CPU time consumed
    pub waiting_on: Option<u32>, // Task ID we're waiting on (for priority inheritance)
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
            base_priority: priority,
            state: ObjectState::Idle,
            context: CpuContext::empty(),
            quantum: Self::priority_to_quantum(priority),
            ticks_remaining: Self::priority_to_quantum(priority),
            cpu_affinity: 0x0F, // All 4 CPUs by default
            total_ticks: 0,
            waiting_on: None,
            mailbox: [Message::empty(); MAX_MESSAGES],
            mailbox_head: 0,
            mailbox_tail: 0,
        }
    }

    /// Higher priority = longer quantum (inverse relationship with value)
    const fn priority_to_quantum(priority: u8) -> u32 {
        let base = 10u32;
        // Priority 0 = highest = 40 ticks
        // Priority 255 = lowest = 5 ticks
        let bonus = ((255 - priority as u32) * 35) / 255;
        base + bonus
    }

    /// Create task with allocated stack and guard page
    pub fn new_with_stack(id: u32, priority: u8, process_id: u32, entry_point: u64) -> Self {
        const STACK_SIZE: usize = 64 * 1024;
        
        unsafe {
            use crate::SMME;
            let smme = SMME.lock();
            
            let stack_base = match smme.allocate(STACK_SIZE + 4096) {
                Ok(addr) => addr,
                Err(_) => 0x100000 + (id as usize * (STACK_SIZE + 4096)),
            };
            
            #[cfg(target_arch = "aarch64")]
            {
                use crate::memory::mmu::Mmu;
                Mmu::setup_stack_guard(stack_base, STACK_SIZE);
            }
            
            let mut context = CpuContext::empty();

            #[cfg(target_arch = "aarch64")]
            {
                let sp = (stack_base + STACK_SIZE - 16) as u64;
                context.sp = sp;
                context.x30 = entry_point;
            }

            #[cfg(target_arch = "x86_64")]
            {
                let mut sp = (stack_base + STACK_SIZE - 16) as u64;
                let ptr = sp as *mut u64;
                sp -= 8;
                *ptr.sub(1) = entry_point;
                context.sp = sp;
            }
            
            Self {
                id,
                process_id,
                priority,
                base_priority: priority,
                state: ObjectState::Ready,
                context,
                quantum: Self::priority_to_quantum(priority),
                ticks_remaining: Self::priority_to_quantum(priority),
                cpu_affinity: 0x0F,
                total_ticks: 0,
                waiting_on: None,
                mailbox: [Message::empty(); MAX_MESSAGES],
                mailbox_head: 0,
                mailbox_tail: 0,
            }
        }
    }

    /// Temporarily boost priority (for priority inheritance)
    pub fn boost_priority(&mut self, new_priority: u8) {
        if new_priority < self.priority {
            self.priority = new_priority;
        }
    }

    /// Restore original priority
    pub fn restore_priority(&mut self) {
        self.priority = self.base_priority;
    }

    pub fn post_message(&mut self, msg: Message) -> Result<(), ()> {
        let next_tail = (self.mailbox_tail + 1) % MAX_MESSAGES;
        if next_tail == self.mailbox_head {
            return Err(());
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

    pub fn has_messages(&self) -> bool {
        self.mailbox_head != self.mailbox_tail
    }
}

/// Priority queue entry (index into objects array)
struct PriorityQueue {
    queues: [[Option<u32>; MAX_OBJECTS / PRIORITY_LEVELS]; PRIORITY_LEVELS],
    counts: [usize; PRIORITY_LEVELS],
}

impl PriorityQueue {
    const fn new() -> Self {
        const NONE: Option<u32> = None;
        const EMPTY_QUEUE: [Option<u32>; MAX_OBJECTS / PRIORITY_LEVELS] = [NONE; MAX_OBJECTS / PRIORITY_LEVELS];
        Self {
            queues: [EMPTY_QUEUE; PRIORITY_LEVELS],
            counts: [0; PRIORITY_LEVELS],
        }
    }

    fn priority_to_level(priority: u8) -> usize {
        // Map 0-255 priority to 0-7 levels
        (priority as usize >> 5).min(PRIORITY_LEVELS - 1)
    }

    fn insert(&mut self, task_id: u32, priority: u8) -> bool {
        let level = Self::priority_to_level(priority);
        if self.counts[level] < MAX_OBJECTS / PRIORITY_LEVELS {
            self.queues[level][self.counts[level]] = Some(task_id);
            self.counts[level] += 1;
            true
        } else {
            false
        }
    }

    fn remove(&mut self, task_id: u32, priority: u8) -> bool {
        let level = Self::priority_to_level(priority);
        for i in 0..self.counts[level] {
            if self.queues[level][i] == Some(task_id) {
                // Shift remaining elements
                for j in i..self.counts[level] - 1 {
                    self.queues[level][j] = self.queues[level][j + 1];
                }
                self.counts[level] -= 1;
                self.queues[level][self.counts[level]] = None;
                return true;
            }
        }
        false
    }

    /// Get highest priority ready task
    fn pop_highest(&mut self) -> Option<u32> {
        for level in 0..PRIORITY_LEVELS {
            if self.counts[level] > 0 {
                let task_id = self.queues[level][0];
                // Shift remaining
                for j in 0..self.counts[level] - 1 {
                    self.queues[level][j] = self.queues[level][j + 1];
                }
                self.counts[level] -= 1;
                self.queues[level][self.counts[level]] = None;
                return task_id;
            }
        }
        None
    }

    fn peek_highest(&self) -> Option<u32> {
        for level in 0..PRIORITY_LEVELS {
            if self.counts[level] > 0 {
                return self.queues[level][0];
            }
        }
        None
    }
}

/// Main Scheduler
pub struct ActiveObjectScheduler {
    pub objects: [Option<ActiveObject>; MAX_OBJECTS],
    pub current_object: AtomicU32,
    pub object_count: usize,
    ready_queue: PriorityQueue,
    scheduler_lock: AtomicBool,
    
    // Statistics
    context_switches: AtomicU64,
    preemptions: AtomicU64,
    idle_ticks: AtomicU64,
}

impl ActiveObjectScheduler {
    pub const fn new() -> Self {
        const NONE: Option<ActiveObject> = None;
        Self {
            objects: [NONE; MAX_OBJECTS],
            current_object: AtomicU32::new(0),
            object_count: 0,
            ready_queue: PriorityQueue::new(),
            scheduler_lock: AtomicBool::new(false),
            context_switches: AtomicU64::new(0),
            preemptions: AtomicU64::new(0),
            idle_ticks: AtomicU64::new(0),
        }
    }

    /// Acquire scheduler lock
    #[inline]
    fn lock(&self) {
        while self.scheduler_lock.compare_exchange_weak(
            false, true, Ordering::Acquire, Ordering::Relaxed
        ).is_err() {
            core::hint::spin_loop();
        }
    }

    /// Release scheduler lock
    #[inline]
    fn unlock(&self) {
        self.scheduler_lock.store(false, Ordering::Release);
    }

    pub fn create_object(&mut self, priority: u8) -> Result<u32, ()> {
        if self.object_count >= MAX_OBJECTS {
            return Err(());
        }
        
        let id = self.object_count as u32;
        self.objects[self.object_count] = Some(ActiveObject::new(id, priority, 0));
        self.object_count += 1;
        
        Ok(id)
    }

    /// Create task with entry point
    pub fn create_task(&mut self, priority: u8, entry_point: u64) -> Result<u32, ()> {
        if self.object_count >= MAX_OBJECTS {
            return Err(());
        }
        
        let id = self.object_count as u32;
        
        // Add to ready queue FIRST to ensure stability
        if !self.ready_queue.insert(id, priority) {
            crate::enterprise::audit::log_security(
                crate::enterprise::audit::AuditSeverity::Critical,
                "Scheduler", "Failed to queue new task: Priority level full."
            );
            return Err(());
        }
        
        let task = ActiveObject::new_with_stack(id, priority, 0, entry_point);
        self.objects[self.object_count] = Some(task);
        self.object_count += 1;
        
        crate::enterprise::audit::log_security(
            crate::enterprise::audit::AuditSeverity::Info,
            "Scheduler", &format!("System task created: ID {}, Priority {}", id, priority)
        );
        
        Ok(id)
    }

    /// Make task ready
    pub fn make_ready(&mut self, task_id: u32) {
        let mut should_insert = false;
        let mut priority = 0u8;
        
        if let Some(Some(obj)) = self.objects.get_mut(task_id as usize) {
            if obj.state != ObjectState::Ready && obj.state != ObjectState::Running {
                obj.state = ObjectState::Ready;
                priority = obj.priority;
                should_insert = true;
            }
        }
        
        if should_insert {
            self.ready_queue.insert(task_id, priority);
        }
    }

    pub fn send_message(&mut self, to: u32, msg: Message) -> Result<(), ()> {
        let mut should_insert = false;
        let mut priority = 0u8;
        
        if let Some(Some(obj)) = self.objects.get_mut(to as usize) {
            let was_idle = obj.state == ObjectState::Idle;
            obj.post_message(msg)?;
            
            if was_idle && obj.state == ObjectState::Ready {
                priority = obj.priority;
                should_insert = true;
            }
        } else {
            return Err(());
        }
        
        if should_insert {
            self.ready_queue.insert(to, priority);
        }
        Ok(())
    }

    /// Tick the scheduler - called by hardware timer
    pub fn tick(&mut self) {
        let idx = self.current_object.load(Ordering::Relaxed) as usize;
        
        // Extract values first to avoid borrow conflict
        let mut should_preempt = false;
        let mut task_id = 0u32;
        let mut task_priority = 0u8;
        
        if let Some(Some(obj)) = self.objects.get_mut(idx) {
            if obj.state == ObjectState::Running {
                obj.total_ticks += 1;
                
                if obj.ticks_remaining > 0 {
                    obj.ticks_remaining -= 1;
                } else {
                    // Time quantum expired - preempt!
                    obj.state = ObjectState::Ready;
                    obj.ticks_remaining = obj.quantum;
                    task_id = obj.id;
                    task_priority = obj.priority;
                    should_preempt = true;
                }
            }
        }
        
        if should_preempt {
            self.ready_queue.insert(task_id, task_priority);
            self.preemptions.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Find highest priority ready task
    fn find_next_task(&self) -> Option<usize> {
        // First check priority queue
        if let Some(task_id) = self.ready_queue.peek_highest() {
            return Some(task_id as usize);
        }
        
        // Fallback: scan all objects
        let mut best_idx: Option<usize> = None;
        let mut best_priority: u8 = u8::MAX;
        
        for (idx, obj_opt) in self.objects.iter().enumerate() {
            if let Some(obj) = obj_opt {
                if obj.state == ObjectState::Ready && obj.priority < best_priority {
                    best_priority = obj.priority;
                    best_idx = Some(idx);
                }
            }
        }
        
        best_idx
    }

    /// Check if preemption needed
    pub fn needs_preemption(&self) -> bool {
        let current_idx = self.current_object.load(Ordering::Relaxed) as usize;
        
        if let Some(Some(current)) = self.objects.get(current_idx) {
            if current.state == ObjectState::Running {
                // Check if there's a higher priority ready task
                if let Some(next_id) = self.ready_queue.peek_highest() {
                    if let Some(Some(next)) = self.objects.get(next_id as usize) {
                        return next.priority < current.priority;
                    }
                }
            }
        }
        
        false
    }

    /// Main scheduling function - priority-based preemptive
    pub fn schedule(&mut self) -> bool {
        self.lock();
        
        let current_idx = self.current_object.load(Ordering::Relaxed) as usize;
        
        // Check preemption FIRST (before mutable borrow)
        let needs_preemption = self.needs_preemption_unlocked(current_idx);
        
        // Check if current task is running
        let is_current_running = if let Some(Some(current)) = self.objects.get(current_idx) {
            current.state == ObjectState::Running
        } else {
            false
        };
        
        // If running and no preemption needed, continue
        if is_current_running && !needs_preemption {
            self.unlock();
            return true;
        }
        
        // Extract current task info to avoid borrow conflict
        let mut should_insert_current = false;
        let mut current_task_id = 0u32;
        let mut current_priority = 0u8;
        
        // Now get mutable borrow and preempt if needed
        if is_current_running {
            if let Some(Some(current)) = self.objects.get_mut(current_idx) {
                // Preempt current task
                current.state = ObjectState::Ready;
                current_task_id = current.id;
                current_priority = current.priority;
                should_insert_current = true;
            }
        }
        
        if should_insert_current {
            self.ready_queue.insert(current_task_id, current_priority);
        }
        
        // Find next task to run
        if let Some(next_idx) = self.find_next_task_unlocked() {
            // Extract next task info
            let mut next_task_id = 0u32;
            let mut next_priority = 0u8;
            let mut should_switch = false;
            
            if let Some(Some(next)) = self.objects.get_mut(next_idx) {
                next_task_id = next.id;
                next_priority = next.priority;
                next.state = ObjectState::Running;
                should_switch = current_idx != next_idx;
            }
            
            // Remove from ready queue after releasing borrow
            self.ready_queue.remove(next_task_id, next_priority);
            
            if should_switch {
                self.current_object.store(next_idx as u32, Ordering::Release);
                self.context_switches.fetch_add(1, Ordering::Relaxed);
                
                // Perform actual context switch
                self.unlock();
                self.do_context_switch(current_idx, next_idx);
                return true;
            }
        } else {
            // No ready tasks - enter idle
            self.idle_ticks.fetch_add(1, Ordering::Relaxed);
            self.unlock();
            
            #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
            {
                use crate::hal;
                hal::get_platform().enter_idle_state();
            }
            return false;
        }
        
        self.unlock();
        true
    }

    fn needs_preemption_unlocked(&self, current_idx: usize) -> bool {
        if let Some(Some(current)) = self.objects.get(current_idx) {
            if let Some(next_id) = self.ready_queue.peek_highest() {
                if let Some(Some(next)) = self.objects.get(next_id as usize) {
                    return next.priority < current.priority;
                }
            }
        }
        false
    }

    fn find_next_task_unlocked(&self) -> Option<usize> {
        if let Some(task_id) = self.ready_queue.peek_highest() {
            return Some(task_id as usize);
        }
        
        // Fallback scan
        let mut best_idx: Option<usize> = None;
        let mut best_priority: u8 = u8::MAX;
        
        for (idx, obj_opt) in self.objects.iter().enumerate() {
            if let Some(obj) = obj_opt {
                if obj.state == ObjectState::Ready && obj.priority < best_priority {
                    best_priority = obj.priority;
                    best_idx = Some(idx);
                }
            }
        }
        
        best_idx
    }

    /// Perform context switch between tasks
    fn do_context_switch(&mut self, from_idx: usize, to_idx: usize) {
        // Use raw pointers to avoid borrow checker issues with simultaneous access
        #[cfg(target_arch = "aarch64")]
        unsafe {
            let objects_ptr = self.objects.as_mut_ptr();
            
            let from_opt = &mut *objects_ptr.add(from_idx);
            let to_opt = &*objects_ptr.add(to_idx);
            
            if let (Some(from), Some(to)) = (from_opt.as_mut(), to_opt.as_ref()) {
                crate::arch::aarch64::__switch_context(
                    &mut from.context,
                    &to.context,
                );
            }
        }
        
        #[cfg(target_arch = "x86_64")]
        {
            let _ = (from_idx, to_idx); // suppress unused warnings
        }
    }

    /// Priority inheritance - boost priority of task holding resource
    pub fn priority_inherit(&mut self, blocker_id: u32, waiter_priority: u8) {
        let mut should_remove = false;
        let mut should_insert = false;
        let mut old_priority = 0u8;
        let mut new_priority = 0u8;
        let mut task_id = 0u32;
        let mut is_ready = false;
        
        if let Some(Some(blocker)) = self.objects.get_mut(blocker_id as usize) {
            if waiter_priority < blocker.priority {
                task_id = blocker.id;
                old_priority = blocker.priority;
                should_remove = true;
                
                // Boost priority
                blocker.boost_priority(waiter_priority);
                new_priority = blocker.priority;
                is_ready = blocker.state == ObjectState::Ready;
                should_insert = is_ready;
            }
        }
        
        if should_remove {
            self.ready_queue.remove(task_id, old_priority);
        }
        if should_insert {
            self.ready_queue.insert(task_id, new_priority);
        }
    }

    /// Restore priority after releasing resource
    pub fn priority_restore(&mut self, task_id: u32) {
        let mut should_update = false;
        let mut old_priority = 0u8;
        let mut new_priority = 0u8;
        
        if let Some(Some(task)) = self.objects.get_mut(task_id as usize) {
            old_priority = task.priority;
            task.restore_priority();
            new_priority = task.priority;
            
            if new_priority != old_priority && task.state == ObjectState::Ready {
                should_update = true;
            }
        }
        
        if should_update {
            self.ready_queue.remove(task_id, old_priority);
            self.ready_queue.insert(task_id, new_priority);
        }
    }

    pub fn stats(&self) -> SchedulerStats {
        let mut idle = 0;
        let mut ready = 0;
        let mut running = 0;
        let mut blocked = 0;
        
        for obj in self.objects.iter().flatten() {
            match obj.state {
                ObjectState::Idle => idle += 1,
                ObjectState::Ready => ready += 1,
                ObjectState::Running => running += 1,
                ObjectState::Blocked | ObjectState::Waiting => blocked += 1,
                _ => {}
            }
        }
        
        SchedulerStats {
            total_objects: self.object_count,
            idle_objects: idle,
            ready_objects: ready,
            running_objects: running,
            blocked_objects: blocked,
            context_switches: self.context_switches.load(Ordering::Relaxed),
            preemptions: self.preemptions.load(Ordering::Relaxed),
            idle_ticks: self.idle_ticks.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SchedulerStats {
    pub total_objects: usize,
    pub idle_objects: usize,
    pub ready_objects: usize,
    pub running_objects: usize,
    pub blocked_objects: usize,
    pub context_switches: u64,
    pub preemptions: u64,
    pub idle_ticks: u64,
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
    fn test_priority_to_quantum() {
        // High priority = longer quantum
        let high_q = ActiveObject::priority_to_quantum(0);
        let low_q = ActiveObject::priority_to_quantum(255);
        assert!(high_q > low_q);
    }

    #[test]
    fn test_priority_queue() {
        let mut pq = PriorityQueue::new();
        
        // Insert tasks with different priorities
        pq.insert(1, 100); // Lower priority
        pq.insert(2, 50);  // Higher priority
        pq.insert(3, 200); // Lowest priority
        
        // Should return task 2 first (highest priority = lowest value)
        assert_eq!(pq.pop_highest(), Some(2));
        assert_eq!(pq.pop_highest(), Some(1));
        assert_eq!(pq.pop_highest(), Some(3));
        assert_eq!(pq.pop_highest(), None);
    }

    #[test]
    fn test_priority_inheritance() {
        let mut scheduler = ActiveObjectScheduler::new();
        
        // Create low priority task
        let low_id = scheduler.create_object(200).unwrap();
        
        // Create high priority task
        let _high_id = scheduler.create_object(10).unwrap();
        
        // Simulate priority inheritance
        scheduler.priority_inherit(low_id, 10);
        
        if let Some(Some(task)) = scheduler.objects.get(low_id as usize) {
            assert_eq!(task.priority, 10); // Should be boosted
            assert_eq!(task.base_priority, 200); // Original saved
        }
        
        // Restore priority
        scheduler.priority_restore(low_id);
        
        if let Some(Some(task)) = scheduler.objects.get(low_id as usize) {
            assert_eq!(task.priority, 200); // Should be restored
        }
    }

    #[test]
    fn test_message_passing() {
        let mut scheduler = ActiveObjectScheduler::new();
        let id = scheduler.create_object(10).unwrap();
        
        let msg = Message { id: 1, data: 42, sender: 0, priority: 0 };
        scheduler.send_message(id, msg).unwrap();
        
        let stats = scheduler.stats();
        assert_eq!(stats.ready_objects, 1);
    }
}
