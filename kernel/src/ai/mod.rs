pub mod npu;            // Akselerator Hardware AI Tensor (PCIe)
pub mod omnilang_bridge;  // Bridge Kompilator OmniLang ke Binari ELF Mesin
pub mod llm;    // [NEW] Phase 22 (v5.3) Local LLM
pub mod genai;  // [NEW] Phase 22 (v5.3) Generative AI
pub mod intent; // [NEW] Phase 27.5 Cognitive Intent Parser
pub mod intent_model; // [NEW] Priority #2 TinyML Model
pub mod fabric; // [NEW] Phase 27.4 AI-Native Industrial Fabric

/// Initialize Universal Intelligence Layer components (Phase 27.x)
pub fn init_intelligence() {
    // log_security diganti dengan println! selama boot kritis untuk menghindari deadlock spinlock audit.
    crate::println!("[AI] Initializing Universal Intelligence Layer v10.2 SUPREME...");
    
    // 1. Initialize Sectoral Fabric
    {
        let mut fabric = fabric::AI_FABRIC.lock();
        // Default to Industrial for military-grade stability demo
        fabric.set_profile(fabric::SectorProfile::Industrial);
        fabric.optimize_workload();
    }

    // 2. Initialize Hardware NPU Driver (PCIe)
    {
        use crate::ai::npu::NpuDriver;
        let mut npu_device = npu::GLOBAL_NPU.lock();
        crate::println!("[AI] Memindai PCIe untuk Hardware Akselerator...");
        match npu_device.init() {
            Ok(_) => crate::println!("[AI] Hardware NPU Aktif."),
            Err(_) => crate::println!("[AI] Hardware NPU tidak ditemukan (Simulation Fallback Aktif)."),
        }
    }

    // 3. Initialize Intent Parser (Phase 27.5)
    {
        let mut _intent = intent::INTENT_PARSER.lock();
        crate::println!("[Intent] Cognitive Listener Active.");
    }

    crate::println!("[AI] Cognitive Sync-Align-Harmony: [ ACTIVE ]");
}
 
