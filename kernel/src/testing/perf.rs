//! Performance Tuning Module (Phase 11.3)
//! Scheduler and memory optimization utilities

use crate::{SMME, SCHEDULER};

/// Performance metrics snapshot
pub struct PerfMetrics {
    pub scheduler_latency_us: u64,
    pub memory_footprint_kb: usize,
    pub tick_duration_ns: u64,
    pub context_switches: u64,
}

impl PerfMetrics {
    pub fn collect() -> Self {
        let smme = SMME.lock();
        let stats = smme.stats();
        let _sched = SCHEDULER.lock();

        Self {
            scheduler_latency_us: 50, // Target: <50μs
            memory_footprint_kb: stats.total_committed / 1024,
            tick_duration_ns: 100_000, // 100μs per tick
            context_switches: 0, // Would read from hardware counter
        }
    }

    pub fn meets_targets(&self) -> bool {
        self.scheduler_latency_us < 50
            && self.memory_footprint_kb < 12 * 1024 // <12MB
    }
}

/// Bug triage entry (Phase 11.2)
#[derive(Debug, Clone)]
pub struct BugReport {
    pub id: usize,
    pub severity: Severity,
    pub title: &'static str,
    pub resolved: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Severity {
    P0, // Critical
    P1, // High
    P2, // Medium
    P3, // Low
}

/// Bug tracker
pub struct BugTracker {
    reports: alloc::vec::Vec<BugReport>,
    next_id: usize,
}

impl BugTracker {
    pub const fn new() -> Self {
        Self {
            reports: alloc::vec::Vec::new(),
            next_id: 1,
        }
    }

    pub fn file_bug(&mut self, severity: Severity, title: &'static str) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.reports.push(BugReport { id, severity, title, resolved: false });
        id
    }

    pub fn resolve(&mut self, id: usize) {
        if let Some(bug) = self.reports.iter_mut().find(|b| b.id == id) {
            bug.resolved = true;
        }
    }

    pub fn p0_count(&self) -> usize {
        self.reports.iter().filter(|b| b.severity == Severity::P0 && !b.resolved).count()
    }

    pub fn all_p0_resolved(&self) -> bool {
        self.p0_count() == 0
    }
}

use spin::Mutex;
pub static BUG_TRACKER: Mutex<BugTracker> = Mutex::new(BugTracker::new());
