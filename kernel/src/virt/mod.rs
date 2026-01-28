//! Virtualization Layer for AetherOS v1.1
//! Process Control Block and Virtual Memory abstractions

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Created,
    Ready,
    Running,
    Blocked,
    Terminated,
}

/// Process Control Block (PCB)
pub struct ProcessControlBlock {
    pub id: u32,
    pub parent_id: u32,
    pub name: [u8; 32],     // Small fixed name
    pub state: ProcessState,
    pub page_table_base: usize, // Physical address of page directory
    pub stack_pointer: usize,
    pub entry_point: usize,
}

impl ProcessControlBlock {
    pub const fn new(id: u32, entry_point: usize) -> Self {
        Self {
            id,
            parent_id: 0,
            name: [0; 32],
            state: ProcessState::Created,
            page_table_base: 0,
            stack_pointer: 0,
            entry_point,
        }
    }

    pub fn set_name(&mut self, name: &[u8]) {
        let len = if name.len() > 32 { 32 } else { name.len() };
        for i in 0..len {
            self.name[i] = name[i];
        }
    }
}

/// Virtual Address Space wrapper
pub struct VirtualSpace {
    pub directory_addr: usize,
}

impl VirtualSpace {
    pub fn new() -> Self {
        Self { directory_addr: 0 }
    }

    pub fn map(&mut self, _virt: usize, _phys: usize, _flags: u32) {
        // Todo: Implement page table walk
    }
}
