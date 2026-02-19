//! Android ART Runtime Stubs (Phase 15.2)
//! Dalvik bytecode interpreter, APK installer, Binder IPC

use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

// ===========================
// Dalvik Bytecode Interpreter
// ===========================

/// Dalvik opcodes (subset)
#[derive(Debug, Clone, Copy)]
pub enum DalvikOp {
    Nop,
    Move(u8, u8),          // move vA, vB
    Const(u8, i32),        // const vA, #int
    ReturnVoid,
    Return(u8),
    Goto(i16),
    IfEq(u8, u8, i16),     // if-eq vA, vB, +CCCC
    Add(u8, u8, u8),       // add-int vA, vB, vC
    Sub(u8, u8, u8),
    Mul(u8, u8, u8),
    InvokeVirtual(u16),    // invoke-virtual {args} method@id
    NewInstance(u8, u16),   // new-instance vA, type@BBBB
}

/// Dalvik VM
pub struct DalvikVm {
    registers: [i32; 16],
    pc: usize,
    code: Vec<DalvikOp>,
    running: bool,
}

impl DalvikVm {
    pub fn new() -> Self {
        Self {
            registers: [0; 16],
            pc: 0,
            code: Vec::new(),
            running: false,
        }
    }

    pub fn load(&mut self, bytecode: Vec<DalvikOp>) {
        self.code = bytecode;
        self.pc = 0;
    }

    pub fn step(&mut self) -> Result<(), &'static str> {
        if self.pc >= self.code.len() { return Err("PC out of bounds"); }
        let op = self.code[self.pc];
        match op {
            DalvikOp::Nop => {}
            DalvikOp::Move(dst, src) => {
                self.registers[dst as usize] = self.registers[src as usize];
            }
            DalvikOp::Const(dst, val) => {
                self.registers[dst as usize] = val;
            }
            DalvikOp::Add(dst, a, b) => {
                self.registers[dst as usize] =
                    self.registers[a as usize] + self.registers[b as usize];
            }
            DalvikOp::Sub(dst, a, b) => {
                self.registers[dst as usize] =
                    self.registers[a as usize] - self.registers[b as usize];
            }
            DalvikOp::Mul(dst, a, b) => {
                self.registers[dst as usize] =
                    self.registers[a as usize] * self.registers[b as usize];
            }
            DalvikOp::IfEq(a, b, offset) => {
                if self.registers[a as usize] == self.registers[b as usize] {
                    self.pc = (self.pc as i16 + offset) as usize;
                    return Ok(());
                }
            }
            DalvikOp::Goto(offset) => {
                self.pc = (self.pc as i16 + offset) as usize;
                return Ok(());
            }
            DalvikOp::ReturnVoid | DalvikOp::Return(_) => {
                self.running = false;
                return Ok(());
            }
            DalvikOp::InvokeVirtual(_) => { /* stub */ }
            DalvikOp::NewInstance(dst, _) => {
                self.registers[dst as usize] = 0; // null reference stub
            }
        }
        self.pc += 1;
        Ok(())
    }

    pub fn run(&mut self) -> Result<i32, &'static str> {
        self.running = true;
        while self.running && self.pc < self.code.len() {
            self.step()?;
        }
        Ok(self.registers[0]) // Return v0
    }
}

// ===========================
// APK Installer
// ===========================

/// APK manifest (simplified AndroidManifest.xml)
#[derive(Debug, Clone)]
pub struct ApkManifest {
    pub package: String,
    pub version_code: u32,
    pub version_name: String,
    pub min_sdk: u32,
    pub main_activity: String,
}

/// Installed Android app
pub struct InstalledApk {
    pub manifest: ApkManifest,
    pub dex_data: Vec<u8>, // classes.dex content
}

/// APK installer
pub struct ApkInstaller {
    installed: BTreeMap<String, InstalledApk>,
}

impl ApkInstaller {
    pub const fn new() -> Self {
        Self { installed: BTreeMap::new() }
    }

    pub fn install(&mut self, manifest: ApkManifest, dex: Vec<u8>) -> Result<(), &'static str> {
        let name = manifest.package.clone();
        self.installed.insert(name, InstalledApk {
            manifest,
            dex_data: dex,
        });
        Ok(())
    }

    pub fn uninstall(&mut self, package: &str) -> Result<(), &'static str> {
        self.installed.remove(package).ok_or("Package not found")?;
        Ok(())
    }

    pub fn list(&self) -> Vec<&str> {
        self.installed.keys().map(|s| s.as_str()).collect()
    }

    pub fn find(&self, package: &str) -> Option<&InstalledApk> {
        self.installed.get(package)
    }
}

// ===========================
// Binder IPC Emulation
// ===========================

/// Binder transaction
#[derive(Debug, Clone)]
pub struct BinderTransaction {
    pub code: u32,
    pub data: Vec<u8>,
    pub reply: Vec<u8>,
}

pub struct BinderDriver {
    transactions: Vec<BinderTransaction>,
}

impl BinderDriver {
    pub const fn new() -> Self {
        Self { transactions: Vec::new() }
    }

    pub fn transact(&mut self, code: u32, data: &[u8]) -> Vec<u8> {
        let tx = BinderTransaction {
            code,
            data: data.to_vec(),
            reply: Vec::new(),
        };
        self.transactions.push(tx);
        Vec::new() // reply stub
    }
}

use spin::Mutex;
pub static APK_INSTALLER: Mutex<ApkInstaller> = Mutex::new(ApkInstaller::new());
