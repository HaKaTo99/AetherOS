//! Distributed Computing Infrastructure
//!
//! This module provides the core components for AetherOS's distributed computing capabilities,
//! enabling seamless task execution and data sharing across multiple devices.
//!
//! # Architecture
//!
//! The distributed system consists of three main components:
//!
//! - **Task Migration**: Lightweight migration of Active Objects between devices
//! - **Distributed Storage**: Eventually-consistent key-value store with replication
//! - **Load Balancing**: Metrics-based task placement and auto-migration
//!
//! # Example
//!
//! ```no_run
//! use aetheros_kernel::distributed::{MIGRATION_MANAGER, KV_STORE, LOAD_BALANCER};
//!
//! // Initialize distributed components
//! unsafe {
//!     MIGRATION_MANAGER.init();
//!     KV_STORE.init();
//!     LOAD_BALANCER.init();
//! }
//!
//! // Check if migration is needed
//! if LOAD_BALANCER.should_migrate() {
//!     let target = LOAD_BALANCER.select_target_device();
//!     // Perform migration...
//! }
//! ```
//!
//! # Design Decisions
//!
//! - **Simplified Migration**: Only Active Objects (message-passing tasks) are migrated,
//!   not full processes with memory state. This ensures stability and reliability.
//! - **Eventual Consistency**: The KV store uses last-write-wins conflict resolution,
//!   prioritizing availability over strong consistency.
//! - **Metrics-Based Balancing**: Load decisions use CPU utilization, task count, and
//!   memory pressure to distribute work effectively.

pub mod migration;
pub mod kvstore;
pub mod loadbalancer;

pub use migration::{MigrationManager, MIGRATION_MANAGER};
pub use kvstore::{KvStore, KV_STORE};
pub use loadbalancer::{LoadBalancer, LOAD_BALANCER};

