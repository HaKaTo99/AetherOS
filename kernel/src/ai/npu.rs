//! Neural Processing Unit (NPU) HAL (Phase 19.2)
//! Abstraction for hardware acceleration of AI workloads.

use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NpuType {
    Tpu,        // Google Coral / Tensor Processing Unit
    Hexagon,    // Qualcomm Hexagon DSP
    AppleNe,    // Apple Neural Engine
    Simulated,  // CPU-based fallback
}

pub struct TensorBuffer {
    pub ptr: usize,
    pub size: usize,
    pub shape: Vec<usize>,
}

pub trait NpuDriver: Send + Sync {
    /// Initialize the NPU hardware
    fn init(&mut self) -> Result<(), &'static str>;

    /// Load a compiled model into NPU memory
    fn load_model(&mut self, model_data: &[u8]) -> Result<u32, &'static str>;

    /// Execute inference on inputs
    fn run_inference(&mut self, model_id: u32, inputs: &[TensorBuffer], outputs: &mut [TensorBuffer]) -> Result<(), &'static str>;

    /// Get NPU capabilities
    fn capabilities(&self) -> NpuType;
}

/// Simulated NPU for Development
struct NpuJob {
    id: usize,
    complexity: u32,
    ticks_remaining: u32,
}

pub struct SimulatedNpu {
    status: bool,
    pending_jobs: Vec<NpuJob>,
    next_id: usize,
}

impl SimulatedNpu {
    pub const fn new() -> Self {
        Self {
            status: false,
            pending_jobs: Vec::new(),
            next_id: 1,
        }
    }

    pub fn submit_job(&mut self, complexity: u32) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.pending_jobs.push(NpuJob {
            id,
            complexity,
            ticks_remaining: complexity,
        });
        id
    }

    pub fn process_step(&mut self) -> Option<usize> {
        // Process head of queue
        if let Some(job) = self.pending_jobs.first_mut() {
            if job.ticks_remaining > 0 {
                job.ticks_remaining -= 1;
                return None;
            }
        }
        
        // Job complete
        if !self.pending_jobs.is_empty() {
             let job = self.pending_jobs.remove(0);
             return Some(job.id);
        }
        None
    }
}

impl NpuDriver for SimulatedNpu {
    fn init(&mut self) -> Result<(), &'static str> {
        self.status = true;
        crate::println!("[NPU] Initialized (Simulated)");
        Ok(())
    }

    fn load_model(&mut self, _model_data: &[u8]) -> Result<u32, &'static str> {
        // In the job queue model, loading a model might just be a "job" or a synchronous setup.
        // For now, we'll return a dummy ID.
        if !self.status { return Err("NPU Not Initialized"); }
        crate::println!("[NPU] Model Loaded: ID 1 (Simulated)");
        Ok(1)
    }

    fn run_inference(&mut self, _model_id: u32, _inputs: &[TensorBuffer], _outputs: &mut [TensorBuffer]) -> Result<(), &'static str> {
        // This is the original run_inference, which simulates a synchronous operation.
        // The job queue logic would typically replace or wrap this for asynchronous processing.
        crate::println!("[NPU] Inference running... Done (12ms)");
        Ok(())
    }

    fn capabilities(&self) -> NpuType {
        NpuType::Simulated
    }
}

pub static GLOBAL_NPU: spin::Mutex<SimulatedNpu> = spin::Mutex::new(SimulatedNpu::new());
