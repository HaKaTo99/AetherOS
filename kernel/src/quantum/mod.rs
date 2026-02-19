//! Quantum Computing Subsystem (Phase 19.3)
//! Simulates QPU operations.

pub mod simulator;
pub mod singularity; // [NEW] Phase 30.1 Singularity Evolution Core
pub use simulator::{QuantumSim, Qubit, Complex, GLOBAL_QPU};
pub use singularity::{EvolutionCore, EVOLUTION_CORE};
