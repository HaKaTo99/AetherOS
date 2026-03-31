//! POSIX Compatibility Layer (Phase 15.1)
//! System call translation, VFS, process management, pthreads

use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

// ===========================
// POSIX Syscall Translation
// ===========================

/// POSIX syscall numbers (Linux AArch64 ABI)
pub const SYS_READ: u64 = 63;
pub const SYS_WRITE: u64 = 64;
pub const SYS_OPENAT: u64 = 56;
pub const SYS_CLOSE: u64 = 57;
pub const SYS_FSTAT: u64 = 80;
pub const SYS_MMAP: u64 = 222;
pub const SYS_MUNMAP: u64 = 215;
pub const SYS_BRK: u64 = 214;
pub const SYS_EXIT: u64 = 93;
pub const SYS_CLONE: u64 = 220;
pub const SYS_WAIT4: u64 = 260;
pub const SYS_EXECVE: u64 = 221;
pub const SYS_GETPID: u64 = 172;
pub const SYS_IOCTL: u64 = 29;

/// POSIX error codes
#[derive(Debug, Clone, Copy)]
pub enum PosixError {
    EPERM = 1,
    ENOENT = 2,
    ESRCH = 3,
    EINTR = 4,
    EIO = 5,
    ENOMEM = 12,
    EACCES = 13,
    EBADF = 9,
    EINVAL = 22,
    ENOSYS = 38,
}

/// Translate Linux syscall to AetherOS native
pub fn translate_syscall(num: u64, a0: u64, a1: u64, a2: u64) -> i64 {
    match num {
        SYS_WRITE => posix_write(a0 as i32, a1 as usize, a2 as usize),
        SYS_READ => posix_read(a0 as i32, a1 as usize, a2 as usize),
        SYS_EXIT => { /* mark process as exited */ 0 }
        SYS_GETPID => 1, // stub
        SYS_BRK => posix_brk(a0 as usize),
        _ => -(PosixError::ENOSYS as i64),
    }
}

fn posix_write(fd: i32, _buf: usize, len: usize) -> i64 {
    match fd {
        1 | 2 => len as i64, // stdout/stderr → serial
        _ => -(PosixError::EBADF as i64),
    }
}

fn posix_read(_fd: i32, _buf: usize, _len: usize) -> i64 {
    0 // EOF for now
}

fn posix_brk(_addr: usize) -> i64 {
    0 // Would adjust heap
}

// ===========================
// Virtual File System (VFS)
// ===========================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FsType { RamFs, Ext4, Fat32, DevFs }

/// VFS inode
#[derive(Debug, Clone)]
pub struct VNode {
    pub name: String,
    pub is_dir: bool,
    pub size: usize,
    pub data: Vec<u8>,
    pub children: Vec<VNode>,
    pub fs_type: FsType,
}

impl VNode {
    pub fn file(name: &str, data: &[u8]) -> Self {
        Self {
            name: String::from(name), is_dir: false,
            size: data.len(), data: data.to_vec(),
            children: Vec::new(), fs_type: FsType::RamFs,
        }
    }

    pub fn dir(name: &str) -> Self {
        Self {
            name: String::from(name), is_dir: true,
            size: 0, data: Vec::new(),
            children: Vec::new(), fs_type: FsType::RamFs,
        }
    }
}

/// Virtual File System
pub struct Vfs {
    _root: VNode,
    fd_table: BTreeMap<i32, String>, // fd -> path
    next_fd: i32,
}

impl Vfs {
    pub fn new() -> Self {
        let mut root = VNode::dir("/");
        root.children.push(VNode::dir("dev"));
        root.children.push(VNode::dir("proc"));
        root.children.push(VNode::dir("tmp"));
        root.children.push(VNode::dir("home"));

        Self { _root: root, fd_table: BTreeMap::new(), next_fd: 3 }
    }

    pub fn open(&mut self, path: &str) -> Result<i32, PosixError> {
        let fd = self.next_fd;
        self.next_fd += 1;
        self.fd_table.insert(fd, String::from(path));
        Ok(fd)
    }

    pub fn close(&mut self, fd: i32) -> Result<(), PosixError> {
        self.fd_table.remove(&fd).ok_or(PosixError::EBADF)?;
        Ok(())
    }

    pub fn read(&self, fd: i32, _buf: &mut [u8]) -> Result<usize, PosixError> {
        let _path = self.fd_table.get(&fd).ok_or(PosixError::EBADF)?;
        // Would look up VNode and read data
        Ok(0)
    }

    pub fn write(&self, fd: i32, data: &[u8]) -> Result<usize, PosixError> {
        let _path = self.fd_table.get(&fd).ok_or(PosixError::EBADF)?;
        Ok(data.len())
    }
}

// ===========================
// Process Management
// ===========================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessState { Running, Sleeping, Zombie, Stopped }

/// POSIX process
pub struct PosixProcess {
    pub pid: u32,
    pub ppid: u32,
    pub state: ProcessState,
    pub exit_code: i32,
    pub name: String,
}

impl PosixProcess {
    pub fn new(pid: u32, ppid: u32, name: &str) -> Self {
        Self {
            pid, ppid,
            state: ProcessState::Running,
            exit_code: 0,
            name: String::from(name),
        }
    }
}

/// fork() stub
pub fn posix_fork(_parent: &PosixProcess) -> PosixProcess {
    static mut NEXT_PID: u32 = 100;
    let pid = unsafe { NEXT_PID += 1; NEXT_PID };
    PosixProcess::new(pid, _parent.pid, &_parent.name)
}

// ===========================
// Pthreads
// ===========================

/// Pthread attribute
pub struct PthreadAttr {
    pub stack_size: usize,
    pub detached: bool,
}

impl PthreadAttr {
    pub fn new() -> Self {
        Self { stack_size: 8192, detached: false }
    }
}

/// Pthread handle
pub struct Pthread {
    pub id: u64,
    pub running: bool,
}

impl Pthread {
    pub fn create(_attr: &PthreadAttr) -> Self {
        static mut NEXT_TID: u64 = 1000;
        let id = unsafe { NEXT_TID += 1; NEXT_TID };
        Self { id, running: true }
    }

    pub fn join(&mut self) {
        self.running = false;
    }
}
