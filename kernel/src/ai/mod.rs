pub mod llm;    // [NEW] Phase 22 (v5.3) Local LLM
pub mod genai;  // [NEW] Phase 22 (v5.3) Generative AI
pub mod intent; // [NEW] Phase 27.5 Cognitive Intent Parser

pub struct NpuDriver;
impl NpuDriver {
    pub fn new() -> Self { Self }
    pub fn init(&mut self) -> Result<(), &'static str> { Ok(()) }
    pub fn process_step(&mut self) -> Option<()> { None }
}

pub static GLOBAL_NPU: spin::Mutex<NpuDriver> = spin::Mutex::new(NpuDriver);
