//! SMME - Symbian-Modern Memory Engine
//! Week 2 Implementation

#![no_std]

pub struct SymbianModernMemoryEngine {
    reserved: usize,
    committed: usize,
    total_ram: usize,
}

#[derive(Debug)]
pub enum AllocationError {
    OutOfMemory,
    InvalidRequest,
}

impl SymbianModernMemoryEngine {
    pub fn new(total_ram: usize) -> Self {
        SymbianModernMemoryEngine {
            reserved: 0,
            committed: 0,
            total_ram,
        }
    }

    /// Phase 1: Reserve virtual address space
    pub fn reserve(&mut self, size: usize) -> Result<(), AllocationError> {
        if self.reserved + size > self.total_ram {
            return Err(AllocationError::OutOfMemory);
        }
        self.reserved += size;
        Ok(())
    }

    /// Phase 2: Commit physical memory
    pub fn commit(&mut self, size: usize) -> Result<usize, AllocationError> {
        if self.committed + size > self.reserved {
            return Err(AllocationError::InvalidRequest);
        }
        self.committed += size;
        Ok(self.committed)
    }

    /// Emergency cleanup (Symbian DNA)
    pub fn emergency_cleanup(&mut self) {
        self.reserved = self.committed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_phase_allocation() {
        let mut smme = SymbianModernMemoryEngine::new(1024 * 1024);
        assert!(smme.reserve(512 * 1024).is_ok());
        assert!(smme.commit(256 * 1024).is_ok());
        assert_eq!(smme.committed, 256 * 1024);
    }
}
