pub mod wasm;
pub mod art;
pub mod apm;          // Package Manager (Phase 14.1)
pub mod appframework; // Application Framework (Phase 14.2)
pub mod omnilang;     // [NEW] OmniLang Native Runtime (Source: HaKaTo99/OmniLang)
pub mod posix;        // [NEW] POSIX Compatibility Layer (Phase 15.1)
pub mod android;      // [NEW] Android ART Runtime (Phase 15.2)
pub mod container;    // [NEW] Container Runtime (Phase 15.3)
pub mod quickjs;      // [NEW] QuickJS Runtime (Phase 16.1)
pub mod ai_agent;     // [NEW] AI Agent Runtime (Phase 16.2)
pub mod database;     // [NEW] Database Runtime (Phase 16.4)
pub mod php;          // [NEW] PHP Runtime (Phase 16.5)
pub mod media;        // [NEW] Media Runtime (Phase 16.6)
pub mod browser; // [NEW] Firefox Container (Phase 20.2)
pub mod gaming;  // [NEW] Phase 21 (v5.2) Gaming
pub mod terminal;     // [NEW] Terminal Runtime (Phase 16.2)
pub mod devtools;     // [NEW] DevTools Runtime (Phase 16.3)

pub use apm::{PackageManager, Package, PackageManifest, PACKAGE_MANAGER};
pub use appframework::{Application, AppMetadata, CalculatorApp};
pub use wasm::WasmRuntime;
pub use container::CONTAINER_RUNTIME;
pub use android::APK_INSTALLER;
pub use quickjs::QuickJsRuntime;
pub use ai_agent::AiAgentRuntime;
pub use database::DatabaseRuntime;
pub use php::PhpRuntime;
pub use media::MediaRuntime;
pub use terminal::TerminalRuntime;
pub use devtools::DevTools;
pub use omnilang::OmniRuntime;
