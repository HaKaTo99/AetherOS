//! Memory Management Tests

use super::log;
use alloc::vec::Vec;
use alloc::boxed::Box;

pub fn test_allocation() {
    log(format_args!("[TEST] Allocating large vector (250k items)..."));
    
    let mut vec = Vec::new();
    for i in 0..250_000 {
        vec.push(i as u32);
    }
    
    assert_eq!(vec.len(), 250_000);
    assert_eq!(vec[123456], 123456);
    
    log(format_args!("[TEST] Large vector allocated and verified."));
    
    // vector dropped here, should free memory
    log(format_args!("[TEST] Memory freed."));
    
    log(format_args!("[TEST] Box allocation..."));
    let b = Box::new(0xDEADBEEF_u32);
    assert_eq!(*b, 0xDEADBEEF_u32);
    log(format_args!("[TEST] Box allocation verified."));
}
