//! Synchronization Primitives for AetherOS
//! Mutex, Semaphore, RwLock, and Condition Variables

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicI32, Ordering};
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};

/// Spinlock - basic mutual exclusion
pub struct SpinLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for SpinLock<T> {}
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        // Acquire spinlock
        while self.locked.compare_exchange_weak(
            false, true, Ordering::Acquire, Ordering::Relaxed
        ).is_err() {
            // Spin with hint
            while self.locked.load(Ordering::Relaxed) {
                core::hint::spin_loop();
            }
        }
        
        SpinLockGuard { lock: self }
    }

    pub fn try_lock(&self) -> Option<SpinLockGuard<'_, T>> {
        if self.locked.compare_exchange(
            false, true, Ordering::Acquire, Ordering::Relaxed
        ).is_ok() {
            Some(SpinLockGuard { lock: self })
        } else {
            None
        }
    }

    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::Relaxed)
    }
}

pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Deref for SpinLockGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> DerefMut for SpinLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> Drop for SpinLockGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

/// Mutex with priority inheritance support
pub struct Mutex<T> {
    locked: AtomicBool,
    owner: AtomicU32,  // Task ID of owner (0 = no owner)
    waiters: AtomicU32, // Count of waiting tasks
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    pub const fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            owner: AtomicU32::new(0),
            waiters: AtomicU32::new(0),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        // Fast path: try to acquire immediately
        if self.locked.compare_exchange(
            false, true, Ordering::Acquire, Ordering::Relaxed
        ).is_ok() {
            // TODO: Set owner to current task ID
            return MutexGuard { mutex: self };
        }

        // Slow path: spin wait (in real OS, would block and trigger priority inheritance)
        self.waiters.fetch_add(1, Ordering::Relaxed);
        
        loop {
            // Check for priority inheritance opportunity
            // In real impl: scheduler.priority_inherit(owner, current_priority)
            
            while self.locked.load(Ordering::Relaxed) {
                core::hint::spin_loop();
            }
            
            if self.locked.compare_exchange(
                false, true, Ordering::Acquire, Ordering::Relaxed
            ).is_ok() {
                self.waiters.fetch_sub(1, Ordering::Relaxed);
                return MutexGuard { mutex: self };
            }
        }
    }

    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        if self.locked.compare_exchange(
            false, true, Ordering::Acquire, Ordering::Relaxed
        ).is_ok() {
            Some(MutexGuard { mutex: self })
        } else {
            None
        }
    }

    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::Relaxed)
    }

    pub fn waiters(&self) -> u32 {
        self.waiters.load(Ordering::Relaxed)
    }
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.owner.store(0, Ordering::Relaxed);
        self.mutex.locked.store(false, Ordering::Release);
        // TODO: Trigger priority restore if needed
    }
}

/// Counting Semaphore
pub struct Semaphore {
    count: AtomicI32,
    max_count: i32,
    waiters: AtomicU32,
}

impl Semaphore {
    pub const fn new(initial: i32) -> Self {
        Self {
            count: AtomicI32::new(initial),
            max_count: i32::MAX,
            waiters: AtomicU32::new(0),
        }
    }

    pub const fn with_max(initial: i32, max: i32) -> Self {
        Self {
            count: AtomicI32::new(initial),
            max_count: max,
            waiters: AtomicU32::new(0),
        }
    }

    /// Acquire (P operation / wait)
    pub fn acquire(&self) {
        self.waiters.fetch_add(1, Ordering::Relaxed);
        
        loop {
            let current = self.count.load(Ordering::Acquire);
            if current > 0 {
                if self.count.compare_exchange_weak(
                    current, current - 1, Ordering::AcqRel, Ordering::Relaxed
                ).is_ok() {
                    self.waiters.fetch_sub(1, Ordering::Relaxed);
                    return;
                }
            } else {
                // Would block - spin (in real OS, would sleep)
                core::hint::spin_loop();
            }
        }
    }

    /// Try to acquire without blocking
    pub fn try_acquire(&self) -> bool {
        loop {
            let current = self.count.load(Ordering::Acquire);
            if current <= 0 {
                return false;
            }
            if self.count.compare_exchange_weak(
                current, current - 1, Ordering::AcqRel, Ordering::Relaxed
            ).is_ok() {
                return true;
            }
        }
    }

    /// Release (V operation / signal)
    pub fn release(&self) {
        loop {
            let current = self.count.load(Ordering::Acquire);
            if current >= self.max_count {
                return; // At max
            }
            if self.count.compare_exchange_weak(
                current, current + 1, Ordering::AcqRel, Ordering::Relaxed
            ).is_ok() {
                return;
            }
        }
    }

    pub fn available(&self) -> i32 {
        self.count.load(Ordering::Relaxed)
    }

    pub fn waiters(&self) -> u32 {
        self.waiters.load(Ordering::Relaxed)
    }
}

/// Binary Semaphore (optimized)
pub struct BinarySemaphore {
    available: AtomicBool,
}

impl BinarySemaphore {
    pub const fn new(available: bool) -> Self {
        Self {
            available: AtomicBool::new(available),
        }
    }

    pub fn wait(&self) {
        loop {
            if self.available.compare_exchange(
                true, false, Ordering::Acquire, Ordering::Relaxed
            ).is_ok() {
                return;
            }
            core::hint::spin_loop();
        }
    }

    pub fn try_wait(&self) -> bool {
        self.available.compare_exchange(
            true, false, Ordering::Acquire, Ordering::Relaxed
        ).is_ok()
    }

    pub fn signal(&self) {
        self.available.store(true, Ordering::Release);
    }

    pub fn is_available(&self) -> bool {
        self.available.load(Ordering::Relaxed)
    }
}

/// Reader-Writer Lock (readers preference)
pub struct RwLock<T> {
    // Positive = number of readers, -1 = writer, 0 = unlocked
    state: AtomicI32,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for RwLock<T> {}
unsafe impl<T: Send + Sync> Sync for RwLock<T> {}

impl<T> RwLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            state: AtomicI32::new(0),
            data: UnsafeCell::new(data),
        }
    }

    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        loop {
            let state = self.state.load(Ordering::Acquire);
            if state >= 0 {
                // No writer, try to add reader
                if self.state.compare_exchange_weak(
                    state, state + 1, Ordering::AcqRel, Ordering::Relaxed
                ).is_ok() {
                    return RwLockReadGuard { lock: self };
                }
            } else {
                // Writer active, spin
                core::hint::spin_loop();
            }
        }
    }

    pub fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
        loop {
            let state = self.state.load(Ordering::Acquire);
            if state < 0 {
                return None; // Writer active
            }
            if self.state.compare_exchange_weak(
                state, state + 1, Ordering::AcqRel, Ordering::Relaxed
            ).is_ok() {
                return Some(RwLockReadGuard { lock: self });
            }
        }
    }

    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        loop {
            if self.state.compare_exchange(
                0, -1, Ordering::Acquire, Ordering::Relaxed
            ).is_ok() {
                return RwLockWriteGuard { lock: self };
            }
            core::hint::spin_loop();
        }
    }

    pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>> {
        if self.state.compare_exchange(
            0, -1, Ordering::Acquire, Ordering::Relaxed
        ).is_ok() {
            Some(RwLockWriteGuard { lock: self })
        } else {
            None
        }
    }

    pub fn reader_count(&self) -> i32 {
        let state = self.state.load(Ordering::Relaxed);
        if state > 0 { state } else { 0 }
    }

    pub fn is_write_locked(&self) -> bool {
        self.state.load(Ordering::Relaxed) < 0
    }
}

pub struct RwLockReadGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<T> Deref for RwLockReadGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> Drop for RwLockReadGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.state.fetch_sub(1, Ordering::Release);
    }
}

pub struct RwLockWriteGuard<'a, T> {
    lock: &'a RwLock<T>,
}

impl<T> Deref for RwLockWriteGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> DerefMut for RwLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> Drop for RwLockWriteGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.state.store(0, Ordering::Release);
    }
}

/// Once - run initialization exactly once
pub struct Once {
    state: AtomicU32,
}

const ONCE_UNINIT: u32 = 0;
const ONCE_RUNNING: u32 = 1;
const ONCE_COMPLETE: u32 = 2;

impl Once {
    pub const fn new() -> Self {
        Self {
            state: AtomicU32::new(ONCE_UNINIT),
        }
    }

    pub fn call_once<F: FnOnce()>(&self, f: F) {
        if self.state.load(Ordering::Acquire) == ONCE_COMPLETE {
            return;
        }

        // Try to become the initializer
        if self.state.compare_exchange(
            ONCE_UNINIT, ONCE_RUNNING, Ordering::Acquire, Ordering::Relaxed
        ).is_ok() {
            f();
            self.state.store(ONCE_COMPLETE, Ordering::Release);
        } else {
            // Wait for completion
            while self.state.load(Ordering::Acquire) != ONCE_COMPLETE {
                core::hint::spin_loop();
            }
        }
    }

    pub fn is_completed(&self) -> bool {
        self.state.load(Ordering::Relaxed) == ONCE_COMPLETE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinlock() {
        let lock = SpinLock::new(42);
        {
            let mut guard = lock.lock();
            *guard = 100;
        }
        assert_eq!(*lock.lock(), 100);
    }

    #[test]
    fn test_mutex() {
        let mutex = Mutex::new(0);
        {
            let mut guard = mutex.lock();
            *guard = 42;
        }
        assert_eq!(*mutex.lock(), 42);
    }

    #[test]
    fn test_semaphore() {
        let sem = Semaphore::new(2);
        
        assert!(sem.try_acquire());
        assert!(sem.try_acquire());
        assert!(!sem.try_acquire()); // Should fail - count is 0
        
        sem.release();
        assert!(sem.try_acquire()); // Should succeed now
    }

    #[test]
    fn test_rwlock() {
        let lock = RwLock::new(42);
        
        // Multiple readers allowed
        {
            let r1 = lock.read();
            let r2 = lock.read();
            assert_eq!(*r1, 42);
            assert_eq!(*r2, 42);
        }
        
        // Writer gets exclusive access
        {
            let mut w = lock.write();
            *w = 100;
        }
        
        assert_eq!(*lock.read(), 100);
    }

    #[test]
    fn test_once() {
        let once = Once::new();
        let counter = core::sync::atomic::AtomicU32::new(0);
        
        once.call_once(|| {
            counter.fetch_add(1, Ordering::Relaxed);
        });
        
        once.call_once(|| {
            counter.fetch_add(1, Ordering::Relaxed);
        });
        
        // Should only run once
        assert_eq!(counter.load(Ordering::Relaxed), 1);
    }
}
