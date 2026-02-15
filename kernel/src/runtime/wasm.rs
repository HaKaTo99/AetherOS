//! WASM Runtime (Phase 15.4)
//! WebAssembly interpreter, WASI interface, sandboxed execution

use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

// ===========================
// WASM Module
// ===========================

/// WASM value types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WasmType { I32, I64, F32, F64 }

/// WASM value
#[derive(Debug, Clone, Copy)]
pub enum WasmValue {
    I32(i32),
    I64(i64),
}

/// WASM instruction (subset)
#[derive(Debug, Clone)]
pub enum WasmInstr {
    I32Const(i32),
    I64Const(i64),
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32Eq,
    I32LtS,
    LocalGet(u32),
    LocalSet(u32),
    Call(u32),
    Return,
    Drop,
    Nop,
}

/// WASM function
#[derive(Debug, Clone)]
pub struct WasmFunc {
    pub name: String,
    pub params: Vec<WasmType>,
    pub results: Vec<WasmType>,
    pub locals: Vec<WasmType>,
    pub body: Vec<WasmInstr>,
}

/// WASM module
#[derive(Debug, Clone)]
pub struct WasmModule {
    pub name: String,
    pub functions: Vec<WasmFunc>,
    pub exports: BTreeMap<String, u32>,
    pub memory_pages: u32,
}

impl WasmModule {
    pub fn from_bytes(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() < 8 { return Err("Too short"); }
        if &data[0..4] != b"\0asm" { return Err("Invalid magic"); }
        if data[4] != 1 { return Err("Unsupported version"); }
        Ok(Self {
            name: String::from("module"),
            functions: Vec::new(),
            exports: BTreeMap::new(),
            memory_pages: 1,
        })
    }
}

// ===========================
// WASM Interpreter
// ===========================

/// Stack-based WASM interpreter with gas metering
pub struct WasmInterpreter {
    stack: Vec<WasmValue>,
    locals: Vec<WasmValue>,
    memory: Vec<u8>,
    gas: u64,
}

impl WasmInterpreter {
    pub fn new(memory_pages: u32) -> Self {
        Self {
            stack: Vec::new(),
            locals: Vec::new(),
            memory: alloc::vec![0u8; memory_pages as usize * 65536],
            gas: 1_000_000,
        }
    }

    pub fn execute(&mut self, func: &WasmFunc) -> Result<Option<WasmValue>, &'static str> {
        self.locals.clear();
        for _ in &func.locals {
            self.locals.push(WasmValue::I32(0));
        }
        self.run_instrs(&func.body)?;
        if func.results.is_empty() { Ok(None) }
        else { self.stack.pop().map(Some).ok_or("Stack underflow") }
    }

    fn run_instrs(&mut self, instrs: &[WasmInstr]) -> Result<(), &'static str> {
        for instr in instrs {
            if self.gas == 0 { return Err("Gas exhausted"); }
            self.gas -= 1;
            match instr {
                WasmInstr::I32Const(v) => self.stack.push(WasmValue::I32(*v)),
                WasmInstr::I64Const(v) => self.stack.push(WasmValue::I64(*v)),
                WasmInstr::I32Add => {
                    let b = self.pop_i32()?; let a = self.pop_i32()?;
                    self.stack.push(WasmValue::I32(a.wrapping_add(b)));
                }
                WasmInstr::I32Sub => {
                    let b = self.pop_i32()?; let a = self.pop_i32()?;
                    self.stack.push(WasmValue::I32(a.wrapping_sub(b)));
                }
                WasmInstr::I32Mul => {
                    let b = self.pop_i32()?; let a = self.pop_i32()?;
                    self.stack.push(WasmValue::I32(a.wrapping_mul(b)));
                }
                WasmInstr::I32DivS => {
                    let b = self.pop_i32()?; let a = self.pop_i32()?;
                    if b == 0 { return Err("Division by zero"); }
                    self.stack.push(WasmValue::I32(a / b));
                }
                WasmInstr::I32Eq => {
                    let b = self.pop_i32()?; let a = self.pop_i32()?;
                    self.stack.push(WasmValue::I32(if a == b { 1 } else { 0 }));
                }
                WasmInstr::I32LtS => {
                    let b = self.pop_i32()?; let a = self.pop_i32()?;
                    self.stack.push(WasmValue::I32(if a < b { 1 } else { 0 }));
                }
                WasmInstr::LocalGet(idx) => {
                    let v = self.locals.get(*idx as usize).copied()
                        .unwrap_or(WasmValue::I32(0));
                    self.stack.push(v);
                }
                WasmInstr::LocalSet(idx) => {
                    let v = self.stack.pop().ok_or("Stack underflow")?;
                    if (*idx as usize) < self.locals.len() {
                        self.locals[*idx as usize] = v;
                    }
                }
                WasmInstr::Drop => { self.stack.pop(); }
                WasmInstr::Return | WasmInstr::Nop => {}
                WasmInstr::Call(_) => { /* inter-function call stub */ }
            }
        }
        Ok(())
    }

    fn pop_i32(&mut self) -> Result<i32, &'static str> {
        match self.stack.pop().ok_or("Stack underflow")? {
            WasmValue::I32(v) => Ok(v),
            _ => Err("Type mismatch"),
        }
    }

    pub fn memory_write(&mut self, offset: usize, data: &[u8]) -> Result<(), &'static str> {
        if offset + data.len() > self.memory.len() { return Err("Out of bounds"); }
        self.memory[offset..offset + data.len()].copy_from_slice(data);
        Ok(())
    }

    pub fn memory_read(&self, offset: usize, len: usize) -> Result<&[u8], &'static str> {
        if offset + len > self.memory.len() { return Err("Out of bounds"); }
        Ok(&self.memory[offset..offset + len])
    }
}

// ===========================
// WASI System Interface
// ===========================

pub struct WasiEnv {
    pub args: Vec<String>,
    pub env_vars: Vec<(String, String)>,
    pub exit_code: Option<i32>,
}

impl WasiEnv {
    pub fn new() -> Self {
        Self { args: Vec::new(), env_vars: Vec::new(), exit_code: None }
    }

    pub fn fd_write(&self, fd: i32, data: &[u8]) -> Result<usize, i32> {
        match fd {
            1 | 2 => {
                // Stdout/Stderr: Write to kernel console
                if let Ok(s) = core::str::from_utf8(data) {
                    crate::print!("{}", s);
                } else {
                    // Fallback for non-utf8 data
                    for b in data {
                        crate::print!("{}", *b as char);
                    }
                }
                Ok(data.len())
            }
            _ => Err(8), // EBADF
        }
    }

    pub fn proc_exit(&mut self, code: i32) {
        crate::println!("[WASI] Process exited with code: {}", code);
        self.exit_code = Some(code);
    }
}

// ===========================
// WASM App Store
// ===========================

#[derive(Debug, Clone)]
pub struct WasmApp {
    pub name: String,
    pub version: String,
    pub wasm_data: Vec<u8>,
}

pub struct WasmAppStore {
    apps: Vec<WasmApp>,
}

impl WasmAppStore {
    pub const fn new() -> Self { Self { apps: Vec::new() } }
    pub fn install(&mut self, app: WasmApp) { self.apps.push(app); }
    pub fn list(&self) -> &[WasmApp] { &self.apps }
    pub fn find(&self, name: &str) -> Option<&WasmApp> {
        self.apps.iter().find(|a| a.name == name)
    }
}

/// Legacy compatibility wrapper
pub struct WasmRuntime;
impl WasmRuntime {
    pub fn new() -> Self { Self }
    pub fn execute(&self, binary: &[u8]) -> Result<(), &'static str> {
        let module = WasmModule::from_bytes(binary)?;
        let mut interp = WasmInterpreter::new(module.memory_pages);
        if let Some(func) = module.functions.first() {
            interp.execute(func)?;
        }
        Ok(())
    }
}

use spin::Mutex;
pub static WASM_STORE: Mutex<WasmAppStore> = Mutex::new(WasmAppStore::new());
