pub mod wasm;
pub mod art;
pub mod apm;          // Package Manager (Phase 14.1)
pub mod appframework; // Application Framework (Phase 14.2)
pub mod aetherscript; // [NEW] AetherScript Compiler (Phase 14.4)
pub mod devtools;     // [NEW] LSP + Profiler (Phase 14.3)
pub mod posix;        // [NEW] POSIX Compatibility Layer (Phase 15.1)
pub mod android;      // [NEW] Android ART Runtime (Phase 15.2)
pub mod container;    // [NEW] Container Runtime (Phase 15.3)

pub use apm::{PackageManager, Package, PackageManifest, PACKAGE_MANAGER};
pub use appframework::{Application, AppMetadata, CalculatorApp};
pub use wasm::WasmRuntime;
pub use container::CONTAINER_RUNTIME;
pub use android::APK_INSTALLER;
