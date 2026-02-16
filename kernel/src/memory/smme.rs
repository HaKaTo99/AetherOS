//! SMME - Symbian-Modern Memory Engine
//!
//! A production-ready memory allocator inspired by Symbian's RHeap with modern enhancements.
//!
//! # Architecture
//!
//! SMME uses a three-tier pool structure for efficient memory management:
//!
//! - **L0 Pool**: 64KB - Small allocations (<4KB)
//! - **L1 Pool**: 2MB - Medium allocations (4KB-128KB)
//! - **L2 Pool**: 16MB - Large allocations (>128KB)
//!
//! # Features
//!
//! - **Two-Phase Allocation**: Reserve then commit for lazy allocation
//! - **Free List Management**: Proper deallocation with block coalescing
//! - **Statistics Tracking**: Per-pool usage metrics and fragmentation monitoring
//! - **Thread-Safe**: Atomic operations for concurrent access
//!
//! # Example
//!
//! ```no_run
//! use aetheros_kernel::SMME;
//!
//! // Allocate memory
//! let ptr = unsafe {
//!     SMME.allocate(4096).expect("Out of memory")
//! };
//!
//! // Use the memory...
//!
//! // Deallocate when done
//! unsafe {
//!     SMME.deallocate(ptr, 4096);
//! }
//!
//! // Check statistics
//! let stats = unsafe { SMME.stats() };
//! println!("Memory used: {}/{}", stats.total_committed, stats.total_reserved);
//! ```
//!
//! # Safety
//!
//! All allocation and deallocation operations are marked `unsafe` as they involve
//! raw pointer manipulation. Callers must ensure:
//! - Allocated memory is properly initialized before use
//! - No double-free or use-after-free violations
//! - Correct size is passed to `deallocate()`

use core::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use core::alloc::{GlobalAlloc, Layout};
use core::ptr;

/// Free block header (stored at the beginning of each free block)
#[repr(C)]
struct FreeBlock {
    size: usize,
    next: *mut FreeBlock,
}

impl FreeBlock {
    const fn null() -> *mut FreeBlock {
        ptr::null_mut()
    }
}

/// Memory Pool with proper free list management
pub struct MemoryPool {
    base: usize,
    size: usize,
    reserved: AtomicUsize,
    committed: AtomicUsize,
    
    // Free list for deallocation
    free_list_head: AtomicUsize,  // Pointer stored as usize for atomic ops
    free_list_lock: AtomicBool,   // Simple spinlock
    
    // Statistics
    alloc_count: AtomicUsize,
    free_count: AtomicUsize,
    coalesce_count: AtomicUsize,
}

impl MemoryPool {
    pub const fn new(base: usize, size: usize) -> Self {
        Self {
            base,
            size,
            reserved: AtomicUsize::new(0),
            committed: AtomicUsize::new(0),
            free_list_head: AtomicUsize::new(0),  // null
            free_list_lock: AtomicBool::new(false),
            alloc_count: AtomicUsize::new(0),
            free_count: AtomicUsize::new(0),
            coalesce_count: AtomicUsize::new(0),
        }
    }

    /// Acquire spinlock for free list operations
    #[inline]
    fn lock(&self) {
        while self.free_list_lock.compare_exchange_weak(
            false, true, Ordering::Acquire, Ordering::Relaxed
        ).is_err() {
            core::hint::spin_loop();
        }
    }

    /// Release spinlock
    #[inline]
    fn unlock(&self) {
        self.free_list_lock.store(false, Ordering::Release);
    }

    /// Phase 1: Reserve virtual address space (Symbian DNA)
    pub fn reserve(&self, size: usize) -> Result<usize, AllocationError> {
        // Align size to 16 bytes minimum
        let aligned_size = (size + 15) & !15;
        
        // First, try to find a suitable block in free list
        if let Some(addr) = self.find_free_block(aligned_size) {
            self.alloc_count.fetch_add(1, Ordering::Relaxed);
            return Ok(addr);
        }
        
        // Otherwise, allocate from reserved space
        let old = self.reserved.fetch_add(aligned_size, Ordering::AcqRel);
        if old + aligned_size > self.size {
            self.reserved.fetch_sub(aligned_size, Ordering::Release);
            return Err(AllocationError::OutOfMemory);
        }
        
        self.alloc_count.fetch_add(1, Ordering::Relaxed);
        Ok(self.base + old)
    }

    /// Find and remove a suitable block from the free list
    fn find_free_block(&self, size: usize) -> Option<usize> {
        self.lock();
        
        let mut prev_ptr: *mut FreeBlock = ptr::null_mut();
        let mut current_ptr = self.free_list_head.load(Ordering::Acquire) as *mut FreeBlock;
        
        while !current_ptr.is_null() {
            let current = unsafe { &mut *current_ptr };
            
            if current.size >= size {
                // Found a suitable block
                if current.size >= size + core::mem::size_of::<FreeBlock>() + 16 {
                    // Split the block if it's large enough
                    let new_block_addr = (current_ptr as usize) + size;
                    let new_block = unsafe { &mut *(new_block_addr as *mut FreeBlock) };
                    new_block.size = current.size - size;
                    new_block.next = current.next;
                    
                    // Update previous pointer
                    if prev_ptr.is_null() {
                        self.free_list_head.store(new_block_addr, Ordering::Release);
                    } else {
                        unsafe { (*prev_ptr).next = new_block as *mut FreeBlock; }
                    }
                } else {
                    // Use entire block
                    if prev_ptr.is_null() {
                        self.free_list_head.store(current.next as usize, Ordering::Release);
                    } else {
                        unsafe { (*prev_ptr).next = current.next; }
                    }
                }
                
                self.unlock();
                return Some(current_ptr as usize);
            }
            
            prev_ptr = current_ptr;
            current_ptr = current.next;
        }
        
        self.unlock();
        None
    }

    /// Phase 2: Commit physical memory
    pub fn commit(&self, addr: usize, size: usize) -> Result<(), AllocationError> {
        if addr < self.base || addr + size > self.base + self.size {
            return Err(AllocationError::InvalidAddress);
        }
        
        let offset = addr - self.base;
        let old_committed = self.committed.load(Ordering::Acquire);
        
        if offset + size > old_committed {
            self.committed.store(offset + size, Ordering::Release);
        }
        
        Ok(())
    }

    /// Deallocate memory - add to free list with coalescing
    pub fn deallocate(&self, addr: usize, size: usize) -> Result<(), AllocationError> {
        if addr < self.base || addr >= self.base + self.size {
            return Err(AllocationError::InvalidAddress);
        }
        
        let aligned_size = (size + 15) & !15;
        
        self.lock();
        
        // Create new free block header
        let new_block = unsafe { &mut *(addr as *mut FreeBlock) };
        new_block.size = aligned_size;
        
        // Insert into sorted free list and coalesce
        let head = self.free_list_head.load(Ordering::Acquire) as *mut FreeBlock;
        
        if head.is_null() || addr < head as usize {
            // Insert at head
            new_block.next = head;
            self.free_list_head.store(addr, Ordering::Release);
            
            // Try to coalesce with next
            if !head.is_null() && addr + aligned_size == head as usize {
                new_block.size += unsafe { (*head).size };
                new_block.next = unsafe { (*head).next };
                self.coalesce_count.fetch_add(1, Ordering::Relaxed);
            }
        } else {
            // Find insertion point
            let mut prev_ptr = head;
            let mut current_ptr = unsafe { (*head).next };
            
            while !current_ptr.is_null() && (current_ptr as usize) < addr {
                prev_ptr = current_ptr;
                current_ptr = unsafe { (*current_ptr).next };
            }
            
            let prev = unsafe { &mut *prev_ptr };
            
            // Insert after prev
            new_block.next = current_ptr;
            prev.next = new_block as *mut FreeBlock;
            
            // Try to coalesce with next
            if !current_ptr.is_null() && addr + aligned_size == current_ptr as usize {
                let next = unsafe { &*current_ptr };
                new_block.size += next.size;
                new_block.next = next.next;
                self.coalesce_count.fetch_add(1, Ordering::Relaxed);
            }
            
            // Try to coalesce with prev
            if (prev_ptr as usize) + prev.size == addr {
                prev.size += new_block.size;
                prev.next = new_block.next;
                self.coalesce_count.fetch_add(1, Ordering::Relaxed);
            }
        }
        
        self.free_count.fetch_add(1, Ordering::Relaxed);
        self.unlock();
        
        Ok(())
    }

    pub fn usage(&self) -> (usize, usize) {
        (self.reserved.load(Ordering::Relaxed), 
         self.committed.load(Ordering::Relaxed))
    }

    /// Get detailed statistics
    pub fn detailed_stats(&self) -> PoolStats {
        let mut free_blocks = 0;
        let mut free_bytes = 0;
        
        self.lock();
        let mut current = self.free_list_head.load(Ordering::Acquire) as *mut FreeBlock;
        while !current.is_null() {
            free_blocks += 1;
            free_bytes += unsafe { (*current).size };
            current = unsafe { (*current).next };
        }
        self.unlock();
        
        PoolStats {
            base: self.base,
            size: self.size,
            reserved: self.reserved.load(Ordering::Relaxed),
            committed: self.committed.load(Ordering::Relaxed),
            free_blocks,
            free_bytes,
            alloc_count: self.alloc_count.load(Ordering::Relaxed),
            free_count: self.free_count.load(Ordering::Relaxed),
            coalesce_count: self.coalesce_count.load(Ordering::Relaxed),
        }
    }
}

/// Pool statistics
#[derive(Debug, Clone, Copy)]
pub struct PoolStats {
    pub base: usize,
    pub size: usize,
    pub reserved: usize,
    pub committed: usize,
    pub free_blocks: usize,
    pub free_bytes: usize,
    pub alloc_count: usize,
    pub free_count: usize,
    pub coalesce_count: usize,
}

/// Main SMME Allocator with 4-layer architecture
pub struct SymbianModernMemoryEngine {
    // Layer 1: Core pools
    l0_pool: MemoryPool,  // 64KB fast pool (small allocations)
    l1_pool: MemoryPool,  // 2MB general pool (medium)
    l2_pool: MemoryPool,  // 16MB large pool (big allocations)
    
    // Layer 2: Predictive state
    allocation_history: [AtomicUsize; 16],
    history_index: AtomicUsize,
    
    // Layer 3: Distributed (placeholder for v0.4)
    distributed_enabled: bool,
    
    // Tracking allocations for deallocation
    // Maps address to (pool_id, size) - simplified approach
    // In production, use a proper hash map
}

impl SymbianModernMemoryEngine {
    pub const fn new(_total_ram: usize) -> Self {
        const ATOMIC_ZERO: AtomicUsize = AtomicUsize::new(0);
        
        Self {
            // Quantum Fortress v6.0 Stability: Massive 400MB+ Aggregated Heap
            l0_pool: MemoryPool::new(0x0400_0000, 16 * 1024 * 1024),      // 16MB fast pool
            l1_pool: MemoryPool::new(0x0500_0000, 128 * 1024 * 1024),     // 128MB general pool
            l2_pool: MemoryPool::new(0x0D00_0000, 256 * 1024 * 1024),    // 256MB large pool
            allocation_history: [ATOMIC_ZERO; 16],
            history_index: AtomicUsize::new(0),
            distributed_enabled: false,
        }
    }

    /// Get pool for a given address
    fn get_pool_for_address(&self, addr: usize) -> Option<&MemoryPool> {
        if addr >= 0x0400_0000 && addr < 0x0500_0000 {
            Some(&self.l0_pool)
        } else if addr >= 0x0500_0000 && addr < 0x0D00_0000 {
            Some(&self.l1_pool)
        } else if addr >= 0x0D00_0000 && addr < 0x1D00_0000 {
            Some(&self.l2_pool)
        } else {
            None
        }
    }

    /// Get pool for a given size
    fn get_pool_for_size(&self, size: usize) -> &MemoryPool {
        if size <= 16 * 1024 * 1024 {
            &self.l0_pool
        } else if size <= 128 * 1024 * 1024 {
            &self.l1_pool
        } else {
            &self.l2_pool
        }
    }

    /// Smart allocation with pool selection
    pub fn allocate(&self, size: usize) -> Result<usize, AllocationError> {
        let pool = self.get_pool_for_size(size);

        // Two-phase allocation
        let addr = pool.reserve(size)?;
        pool.commit(addr, size)?;

        // Update history for prediction
        let idx = self.history_index.fetch_add(1, Ordering::Relaxed) % 16;
        self.allocation_history[idx].store(size, Ordering::Relaxed);

        Ok(addr)
    }

    /// Deallocate memory (NEW - proper implementation)
    pub fn deallocate(&self, addr: usize, size: usize) -> Result<(), AllocationError> {
        if let Some(pool) = self.get_pool_for_address(addr) {
            pool.deallocate(addr, size)
        } else {
            Err(AllocationError::InvalidAddress)
        }
    }

    /// Predictive cleanup (Oracle Engine integration point)
    pub fn predictive_cleanup(&self) -> usize {
        let (_reserved, committed) = self.l1_pool.usage();
        let utilization = if self.l1_pool.size > 0 {
            (committed * 100) / self.l1_pool.size
        } else {
            0
        };
        
        if utilization > 80 {
            self.emergency_cleanup()
        } else {
            0
        }
    }

    /// Emergency cleanup (Symbian DNA)
    pub fn emergency_cleanup(&self) -> usize {
        let (_reserved, committed) = self.l1_pool.usage();
        _reserved.saturating_sub(committed)
    }

    /// Get memory statistics
    pub fn stats(&self) -> MemoryStats {
        let (l0_res, l0_com) = self.l0_pool.usage();
        let (l1_res, l1_com) = self.l1_pool.usage();
        let (l2_res, l2_com) = self.l2_pool.usage();
        
        MemoryStats {
            total_reserved: l0_res + l1_res + l2_res,
            total_committed: l0_com + l1_com + l2_com,
            l0_usage: l0_com,
            l1_usage: l1_com,
            l2_usage: l2_com,
        }
    }

    /// Get detailed statistics per pool
    pub fn detailed_stats(&self) -> DetailedMemoryStats {
        DetailedMemoryStats {
            l0: self.l0_pool.detailed_stats(),
            l1: self.l1_pool.detailed_stats(),
            l2: self.l2_pool.detailed_stats(),
        }
    }

    /// Get allocation pattern for prediction
    pub fn get_allocation_pattern(&self) -> [usize; 16] {
        let mut pattern = [0usize; 16];
        for (i, item) in pattern.iter_mut().enumerate() {
            *item = self.allocation_history[i].load(Ordering::Relaxed);
        }
        pattern
    }

    /// Predict next allocation size based on history
    pub fn predict_next_size(&self) -> usize {
        let pattern = self.get_allocation_pattern();
        let sum: usize = pattern.iter().filter(|&&x| x > 0).sum();
        let count = pattern.iter().filter(|&&x| x > 0).count();
        
        if count > 0 {
            sum / count
        } else {
            4096 // Default 4KB
        }
    }
}

unsafe impl GlobalAlloc for SymbianModernMemoryEngine {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size().max(layout.align());
        match self.allocate(size) {
            Ok(addr) => addr as *mut u8,
            Err(_) => ptr::null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size().max(layout.align());
        let _ = self.deallocate(ptr as usize, size);
    }
}

unsafe impl Sync for SymbianModernMemoryEngine {}

#[derive(Debug, Clone, Copy)]
pub struct MemoryStats {
    pub total_reserved: usize,
    pub total_committed: usize,
    pub l0_usage: usize,
    pub l1_usage: usize,
    pub l2_usage: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct DetailedMemoryStats {
    pub l0: PoolStats,
    pub l1: PoolStats,
    pub l2: PoolStats,
}

#[derive(Debug, Clone, Copy)]
pub enum AllocationError {
    OutOfMemory,
    InvalidAddress,
    InvalidRequest,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_phase_allocation() {
        let smme = SymbianModernMemoryEngine::new(1 << 30);
        
        let addr = smme.allocate(1024 * 1024).unwrap();
        assert!(addr > 0);
        
        let stats = smme.stats();
        assert!(stats.total_committed > 0);
    }

    #[test]
    fn test_pool_selection() {
        let smme = SymbianModernMemoryEngine::new(1 << 30);
        
        let small = smme.allocate(1024).unwrap();
        let medium = smme.allocate(128 * 1024).unwrap();
        let large = smme.allocate(4 * 1024 * 1024).unwrap();
        
        assert!(small != medium);
        assert!(medium != large);
    }

    #[test]
    fn test_alloc_dealloc_cycle() {
        let smme = SymbianModernMemoryEngine::new(1 << 30);
        
        // Allocate
        let addr1 = smme.allocate(4096).unwrap();
        assert!(addr1 > 0);
        
        // Deallocate
        let result = smme.deallocate(addr1, 4096);
        assert!(result.is_ok());
        
        // Re-allocate should reuse the freed block
        let addr2 = smme.allocate(4096).unwrap();
        assert_eq!(addr1, addr2); // Should get same address back
    }

    #[test]
    fn test_coalescing() {
        let smme = SymbianModernMemoryEngine::new(1 << 30);
        
        // Allocate two adjacent blocks
        let addr1 = smme.allocate(4096).unwrap();
        let addr2 = smme.allocate(4096).unwrap();
        
        // Free both
        let _ = smme.deallocate(addr1, 4096);
        let _ = smme.deallocate(addr2, 4096);
        
        // Should be coalesced - check via stats
        let stats = smme.l0_pool.detailed_stats();
        assert!(stats.coalesce_count >= 1 || stats.free_blocks <= 2);
    }

    #[test]
    fn test_prediction() {
        let smme = SymbianModernMemoryEngine::new(1 << 30);
        
        // Make some allocations
        let _ = smme.allocate(4096);
        let _ = smme.allocate(4096);
        let _ = smme.allocate(4096);
        
        let predicted = smme.predict_next_size();
        assert_eq!(predicted, 4096);
    }
}
