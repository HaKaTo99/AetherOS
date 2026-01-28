//! Unit Test Framework for Kernel Primitives
//! 
//! Simple test framework that works in no_std environment

/// Test result
#[derive(Debug, PartialEq)]
pub enum TestResult {
    Pass,
    Fail(&'static str),
}

/// Test case
pub struct TestCase {
    pub name: &'static str,
    pub func: fn() -> TestResult,
}

/// Run all tests
pub fn run_tests(tests: &[TestCase]) -> (usize, usize) {
    let mut passed = 0;
    let mut failed = 0;

    for test in tests {
        let result = (test.func)();
        match result {
            TestResult::Pass => {
                passed += 1;
                test_log(&format!("✓ {}", test.name));
            }
            TestResult::Fail(msg) => {
                failed += 1;
                test_log(&format!("✗ {} - {}", test.name, msg));
            }
        }
    }

    (passed, failed)
}

/// Log test output
fn test_log(msg: &str) {
    unsafe {
        let platform = crate::hal::get_platform();
        for byte in msg.bytes() {
            platform.put_char(byte);
        }
        platform.put_char(b'\n');
    }
}

/// Assert macro for tests
#[macro_export]
macro_rules! assert_test {
    ($cond:expr) => {
        if !$cond {
            return TestResult::Fail(stringify!($cond));
        }
    };
    ($cond:expr, $msg:expr) => {
        if !$cond {
            return TestResult::Fail($msg);
        }
    };
}

// Example kernel primitive tests
#[cfg(test)]
mod tests {
    use super::*;

    fn test_smme_allocation() -> TestResult {
        // Test SMME allocator
        TestResult::Pass
    }

    fn test_scheduler_creation() -> TestResult {
        // Test scheduler task creation
        TestResult::Pass
    }

    fn test_paging_mapping() -> TestResult {
        // Test memory paging
        TestResult::Pass
    }

    pub const KERNEL_TESTS: &[TestCase] = &[
        TestCase {
            name: "SMME Allocation",
            func: test_smme_allocation,
        },
        TestCase {
            name: "Scheduler Creation",
            func: test_scheduler_creation,
        },
        TestCase {
            name: "Paging Mapping",
            func: test_paging_mapping,
        },
    ];
}
