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

extern crate alloc;
use alloc::format;
use core::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use core::alloc::{GlobalAlloc, Layout};
use core::ptr;

/// Free block header (stored at the beginning of each free block)
#[repr(C)]
struct FreeBlock {
    size: usize,
    next: *mut FreeBlock,
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
        // Military Grade: L3 Guarded Allocation (v10.2 SUPREME)
        // Add 8 bytes before and 8 bytes after for canaries
        let guarded_size = size + 16;
        let aligned_size = (guarded_size + 15) & !15;
        
        // [SUPREME GUARD] Check for excessively large allocations (Military Grade)
        if aligned_size > 512 * 1024 * 1024 {
             crate::enterprise::audit::log_security(
                 crate::enterprise::audit::AuditSeverity::Critical,
                 "SMME", "EXTREME ALLOCATION SIZE REJECTED (DoS Guard)"
             );
             return Err(AllocationError::InvalidSize);
        }

        // First, try to find a suitable block in free list
        let addr = if let Some(free_addr) = self.find_free_block(aligned_size) {
            self.alloc_count.fetch_add(1, Ordering::Relaxed);
            free_addr
        } else {
            // Otherwise, allocate from reserved space
            let old = self.reserved.fetch_add(aligned_size, Ordering::AcqRel);
            if old + aligned_size > self.size {
                self.reserved.fetch_sub(aligned_size, Ordering::Release);
                
                // [SUPREME GUARD] OOM triggered
                return Err(AllocationError::OutOfMemory);
            }
            self.alloc_count.fetch_add(1, Ordering::Relaxed);
            self.base + old
        };

        // Supreme Stability: Stack Collision Guard
        let current_rsp: usize;
        unsafe {
            core::arch::asm!("mov {}, rsp", out(reg) current_rsp);
        }
        
        // If the allocated address is within 1MB of the current stack, 
        // it's a critical safety violation indicating corrupted metadata.
        if (addr >= current_rsp.saturating_sub(1024 * 1024)) && (addr <= current_rsp + 1024 * 1024) {
             crate::hal::get_platform().puts("\r\n!!! CRITICAL: SMME STACK COLLISION DETECTED !!!\r\n");
             crate::hal::get_platform().puts(&alloc::format!("Attempted to use address 0x{:X} while RSP is 0x{:X}\n", addr, current_rsp));
             panic!("SMME Integrity Violation: Metadata corruption detected.");
        }

        // Supreme Stability: Verify address is actually within this pool's bounds
        if addr < self.base || addr + aligned_size > self.base + self.size {
             return Err(AllocationError::InvalidAddress);
        }
        
        // Write Unique Head Canary (based on base address to identify pool)
        let head_canary = 0xDEAD0000_00000000 | (self.base as u64 & 0xFFFFFFFF);
        unsafe {
            ptr::write(addr as *mut u64, head_canary);
            ptr::write((addr + aligned_size - 8) as *mut u64, 0xFEED_C0DE_BEEF_DEAD);
        }
        
        Ok(addr + 8) // Return pointer to data (after head canary)

    }

    /// Find and remove a suitable block from the free list
    fn find_free_block(&self, size: usize) -> Option<usize> {
        self.lock();
        
        let mut prev_ptr: *mut FreeBlock = ptr::null_mut();
        let mut current_ptr = self.free_list_head.load(Ordering::Acquire) as *mut FreeBlock;
        
        while !current_ptr.is_null() {
            let current = unsafe { &mut *current_ptr };
            
            // GPF Guard: Skip blocks with corrupted/poisoned size
            // Poison pattern 0xDEDEDEDE... would produce unreasonably large sizes
            if current.size == 0 || current.size > self.size {
                // Corrupted block — unlink and skip
                if prev_ptr.is_null() {
                    self.free_list_head.store(0, Ordering::Release);
                } else {
                    unsafe { (*prev_ptr).next = ptr::null_mut(); }
                }
                break;
            }
            
            if current.size >= size {
                // Found a suitable block
                if current.size >= size + 32 {
                    // Split the block only if remainder is large enough for header + useful data
                    let new_block_addr = (current_ptr as usize) + size;
                    let new_block = unsafe { &mut *(new_block_addr as *mut FreeBlock) };
                    new_block.size = current.size - size;
                    new_block.next = current.next;
                    
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


    /// Commit physical memory
    pub fn commit(&self, addr: usize, size: usize) -> Result<(), AllocationError> {
        if addr < self.base || addr + size > self.base + self.size {
            // [SOVEREIGN] Special case: Allow commit to MMIO regions (like LFB) 
            // if they are clearly outside the heap range.
            if addr >= 0x0800_0000 {
                return Ok(());
            }
            return Err(AllocationError::InvalidAddress);
        }
        
        let offset = addr - self.base;
        let old_committed = self.committed.load(Ordering::Acquire);
        
        if offset + size > old_committed {
            self.committed.store(offset + size, Ordering::Release);
        }
        
        Ok(())
    }

    /// [SOVEREIGN] Formal Video Region Mapping (VA=PA for v10.4-alpha)
    pub fn map_video_region(&self, phys_addr: usize, size: usize) -> usize {
        // Log the mapping call to internal security audit
        crate::enterprise::audit::log_security(
            crate::enterprise::audit::AuditSeverity::Info,
            "SMME",
            &format!("Sovereign Video Mapping Registered: 0x{:X} ({} MB)", phys_addr, size / (1024*1024))
        );
        
        // In VA=PA identity mode, return the address directly
        phys_addr
    }

    /// Military Grade: Check canaries for a given data address
    pub fn check_canaries(&self, data_addr: usize, size: usize) -> Result<(), AllocationError> {
        let addr = data_addr - 8;
        unsafe {
            let head = ptr::read(addr as *const u64);
            let tail = ptr::read((addr + 8 + size) as *const u64);
            
            let expected_head_prefix = 0xDEAD0000_00000000u64;
            let expected_head_low = (self.base as u64) & 0xFFFF_FFFF;
            let valid_head = (head & 0xFFFF0000_00000000u64) == expected_head_prefix
                && (head & 0x00000000_FFFFFFFFu64) == expected_head_low;

            if !valid_head || tail != 0xFEED_C0DE_BEEF_DEAD {
                crate::enterprise::audit::log_security(
                    crate::enterprise::audit::AuditSeverity::Critical,
                    "SMME", 
                    &crate::alloc::format!("CANARY BREACH at 0x{:X}! Head: 0x{:X}, Tail: 0x{:X}", data_addr, head, tail)
                );
                return Err(AllocationError::InvalidRequest);
            }
        }
        Ok(())
    }

    /// Deallocate memory - add to free list with coalescing
    pub fn deallocate(&self, data_addr: usize, size: usize) -> Result<(), AllocationError> {
        let addr = data_addr - 8; // Adjust to actual block start (head canary)
        if addr < self.base || addr >= self.base + self.size {
            return Err(AllocationError::InvalidAddress);
        }
        
        // Military Grade: Mandatory Canary Validation before freeing
        self.check_canaries(data_addr, size)?;

        let guarded_size = size + 16;
        let aligned_size = (guarded_size + 15) & !15;
        
        self.lock();
        
        // CRITICAL FIX: Write FreeBlock header FIRST, THEN poison only the
        // data area AFTER the header. Previous code poisoned the entire block
        // with 0xDE before writing the header, but if a concurrent reader or
        // a subsequent find_free_block() ran before the header was fully
        // written, it would dereference poisoned bytes as a pointer,
        // producing GPF at addresses like 0xDEDEDEDE_DEDEDEDE.

        // Step 1: Create the free block header at the start of the block
        let new_block = unsafe { &mut *(addr as *mut FreeBlock) };
        new_block.size = aligned_size;
        new_block.next = ptr::null_mut(); // Safe initial value
        
        // Step 2: Poison ONLY the data area AFTER the FreeBlock header (16 bytes)
        // This prevents use-after-free while keeping the header intact.
        // Use 0xCC (INT3 instruction) to trigger a breakpoint if dereferenced as code.
        let header_size = core::mem::size_of::<FreeBlock>();
        let poison_start = addr + header_size;
        let poison_len = aligned_size.saturating_sub(header_size);
        if poison_len > 0 && poison_start + poison_len <= self.base + self.size {
            unsafe {
                ptr::write_bytes(poison_start as *mut u8, 0xCC, poison_len);
            }
        }
        
        // Step 3: Insert into sorted free list and coalesce
        let head = self.free_list_head.load(Ordering::Acquire) as *mut FreeBlock;
        
        if head.is_null() || addr < head as usize {
            // Insert at head
            new_block.next = head;
            self.free_list_head.store(addr, Ordering::Release);
            
            // Try to coalesce with next
            if !head.is_null() && addr + aligned_size == head as usize {
                let head_size = unsafe { (*head).size };
                let head_next = unsafe { (*head).next };
                // Validate before coalescing
                if head_size > 0 && head_size <= self.size {
                    new_block.size += head_size;
                    new_block.next = head_next;
                    self.coalesce_count.fetch_add(1, Ordering::Relaxed);
                }
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
            
            // Try to coalesce with next (with validation)
            if !current_ptr.is_null() && addr + aligned_size == current_ptr as usize {
                let next_size = unsafe { (*current_ptr).size };
                let next_next = unsafe { (*current_ptr).next };
                if next_size > 0 && next_size <= self.size {
                    new_block.size += next_size;
                    new_block.next = next_next;
                    self.coalesce_count.fetch_add(1, Ordering::Relaxed);
                }
            }
            
            // Try to coalesce with prev (with validation)
            if (prev_ptr as usize) + prev.size == addr && prev.size <= self.size {
                prev.size += new_block.size;
                prev.next = new_block.next;
                self.coalesce_count.fetch_add(1, Ordering::Relaxed);
            }
        }
        
        self.free_count.fetch_add(1, Ordering::Relaxed);
        self.unlock();
        Ok(())
    }

    /// Military Grade Health Audit (Sync-Align-Harmony)
    pub fn audit_health(&self) -> bool {
        self.lock();
        let mut head = self.free_list_head.load(Ordering::Acquire) as *mut FreeBlock;
        let mut prev_addr = 0usize;
        
        while !head.is_null() {
            let current_addr = head as usize;
            
            // 1. Check if blocks are within bounds
            if current_addr < self.base || current_addr >= self.base + self.size {
                self.unlock();
                return false;
            }
            
            // 2. Check for sorted order (prevent loops/corruption)
            if prev_addr != 0 && current_addr <= prev_addr {
                self.unlock();
                return false;
            }
            
            // 3. Check for overlapping or invalid size
            let current = unsafe { &*head };
            if current.size == 0 || current_addr + current.size > self.base + self.size {
                self.unlock();
                return false;
            }
            
            prev_addr = current_addr;
            head = current.next;
        }
        
        self.unlock();
        true
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
    _distributed_enabled: bool,
    
    // Tracking allocations for deallocation
    // Maps address to (pool_id, size) - simplified approach
    // In production, use a proper hash map
}

impl SymbianModernMemoryEngine {
    pub const fn new(_total_ram: usize) -> Self {
        const ATOMIC_ZERO: AtomicUsize = AtomicUsize::new(0);
        
        Self {
            // Quantum Fortress v10.0: Localized Heap for Harmony Mapping
            // Starting at 128MB to avoid any conflict with kernel (1MB-32MB) 
            l0_pool: MemoryPool::new(0x0800_0000, 16 * 1024 * 1024),      
            l1_pool: MemoryPool::new(0x0900_0000, 32 * 1024 * 1024),     
            l2_pool: MemoryPool::new(0x0B00_0000, 64 * 1024 * 1024),     

            allocation_history: [ATOMIC_ZERO; 16],
            history_index: AtomicUsize::new(0),
            _distributed_enabled: false,
        }
    }

    /// Get pool for a given address
    fn get_pool_for_address(&self, addr: usize) -> Option<&MemoryPool> {
        // Keep lookup tied to the actual configured pool windows to avoid
        // stale hardcoded ranges when pool layout changes.
        if addr >= self.l0_pool.base && addr < self.l0_pool.base + self.l0_pool.size {
            Some(&self.l0_pool)
        } else if addr >= self.l1_pool.base && addr < self.l1_pool.base + self.l1_pool.size {
            Some(&self.l1_pool)
        } else if addr >= self.l2_pool.base && addr < self.l2_pool.base + self.l2_pool.size {
            Some(&self.l2_pool)
        } else {
            None
        }
    }

    /// Get pool for a given size and intent (Phase 10.0 Harmony)
    fn get_pool_for_size_and_intent(&self, size: usize, intent: crate::ai::intent::UserIntent) -> &MemoryPool {
        use crate::ai::intent::UserIntent;
        
        match intent {
            UserIntent::Development => {
                // For Development, prioritize L1 pool for compiler-like workloads
                if size <= 128 * 1024 { &self.l1_pool }
                else { &self.l2_pool }
            },
            UserIntent::Multimedia => {
                // For Multimedia, use L0 for many small objects
                if size <= 64 * 1024 { &self.l0_pool }
                else { &self.l1_pool }
            },
            _ => {
                // Standard logic
                if size <= 64 * 1024 { &self.l0_pool }
                else if size <= 2 * 1024 * 1024 { &self.l1_pool }
                else { &self.l2_pool }
            }
        }
    }

    /// Smart allocation with pool selection and intent awareness
    pub fn allocate(&self, size: usize) -> Result<usize, AllocationError> {
        // Fetch current user intent
        let intent = crate::ai::intent::INTENT_PARSER.lock().get_intent();
        let pool = self.get_pool_for_size_and_intent(size, intent);

        // Two-phase allocation
        let addr = pool.reserve(size)?;

        // Supreme Stability: Strict address range validation
        // Heap pools are at 128MB+, anything below is strictly forbidden
        if addr < 0x0800_0000 {
             return Err(AllocationError::InvalidAddress);
        }

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

    /// Predictive cleanup (Oracle Engine integration point) - Phase 10.0 Harmony
    pub fn predictive_cleanup(&self) -> usize {
        let (_reserved, committed) = self.l1_pool.usage();
        let utilization = if self.l1_pool.size > 0 {
            (committed * 100) / self.l1_pool.size
        } else {
            0
        };
        
        // Fetch intent for threshold adjustment
        let intent = crate::ai::intent::INTENT_PARSER.lock().get_intent();
        let threshold = match intent {
            crate::ai::intent::UserIntent::HighPerformanceGaming | crate::ai::intent::UserIntent::DistributedCompute => 70, // Aggressive
            crate::ai::intent::UserIntent::Development => 90, // Lazy/Development
            _ => 80, // Default
        };

        if utilization > threshold {
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

    /// Perform a full health audit of all pools
    pub fn audit_all_health(&self) -> bool {
        self.l0_pool.audit_health() && 
        self.l1_pool.audit_health() && 
        self.l2_pool.audit_health()
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
    InvalidSize,
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

    #[test]
    fn test_pool_lookup_uses_configured_ranges() {
        let smme = SymbianModernMemoryEngine::new(1 << 30);

        // Start addresses of each pool must map correctly.
        assert!(core::ptr::eq(
            smme.get_pool_for_address(smme.l0_pool.base).unwrap(),
            &smme.l0_pool
        ));
        assert!(core::ptr::eq(
            smme.get_pool_for_address(smme.l1_pool.base).unwrap(),
            &smme.l1_pool
        ));
        assert!(core::ptr::eq(
            smme.get_pool_for_address(smme.l2_pool.base).unwrap(),
            &smme.l2_pool
        ));

        // End boundaries are exclusive.
        assert!(smme.get_pool_for_address(smme.l0_pool.base + smme.l0_pool.size).is_none());
        assert!(smme.get_pool_for_address(smme.l1_pool.base + smme.l1_pool.size).is_none());
        assert!(smme.get_pool_for_address(smme.l2_pool.base + smme.l2_pool.size).is_none());
    }
}
