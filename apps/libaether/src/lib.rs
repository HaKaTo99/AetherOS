#![no_std]

use core::arch::asm;

/// AetherOS Syscall Numbers
pub const SYS_READ: usize = 0;
pub const SYS_WRITE: usize = 1;
pub const SYS_OPEN: usize = 2;
pub const SYS_CLOSE: usize = 3;
pub const SYS_SCHEME_OPEN: usize = 10;
pub const SYS_SCHEME_READ: usize = 11;
pub const SYS_SCHEME_WRITE: usize = 12;
pub const SYS_MMAP: usize = 90;
pub const SYS_EXIT: usize = 60;
pub const SYS_SPAWN: usize = 100;

/// RGB Color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Self = Self::new(0, 0, 0);
    pub const WHITE: Self = Self::new(255, 255, 255);
    
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub const fn from_hex(hex: u32) -> Self {
        Self::new(
            ((hex >> 16) & 0xFF) as u8,
            ((hex >> 8) & 0xFF) as u8,
            (hex & 0xFF) as u8,
        )
    }

    pub fn lerp(a: Self, b: Self, t: u8) -> Self {
        let t = t as u32;
        let inv_t = 255 - t;
        let r = (a.r as u32 * inv_t + b.r as u32 * t) / 255;
        let g = (a.g as u32 * inv_t + b.g as u32 * t) / 255;
        let b = (a.b as u32 * inv_t + b.b as u32 * t) / 255;
        Self::new(r as u8, g as u8, b as u8)
    }
}

/// Rectangle area
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self { x, y, width, height }
    }

    pub fn contains(&self, p: Point) -> bool {
        p.x >= self.x && p.x < self.x + self.width && p.y >= self.y && p.y < self.y + self.height
    }
}

/// AetherUI Rendering Utilities
pub struct Renderer<'a> {
    pub fb: &'a mut [u32],
    pub width: usize,
    pub height: usize,
}

impl<'a> Renderer<'a> {
    pub fn new(fb: &'a mut [u32], width: usize, height: usize) -> Self {
        Self { fb, width, height }
    }

    pub fn draw_rect(&mut self, rect: Rect, color: Color) {
        let c = 0xFF000000 | ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32);
        for y in rect.y..core::cmp::min(rect.y + rect.height, self.height) {
            for x in rect.x..core::cmp::min(rect.x + rect.width, self.width) {
                self.fb[y * self.width + x] = c;
            }
        }
    }

    pub fn draw_rounded_rect(&mut self, rect: Rect, radius: usize, color: Color) {
        let c = 0xFF000000 | ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32);
        for y in rect.y..core::cmp::min(rect.y + rect.height, self.height) {
            for x in rect.x..core::cmp::min(rect.x + rect.width, self.width) {
                let dx = if x < rect.x + radius { rect.x + radius - x } 
                        else if x >= rect.x + rect.width - radius { x - (rect.x + rect.width - radius - 1) } 
                        else { 0 };
                let dy = if y < rect.y + radius { rect.y + radius - y } 
                        else if y >= rect.y + rect.height - radius { y - (rect.y + rect.height - radius - 1) } 
                        else { 0 };
                
                if dx * dx + dy * dy <= radius * radius {
                    self.fb[y * self.width + x] = c;
                }
            }
        }
    }

    pub fn draw_shadow(&mut self, rect: Rect, radius: usize, intensity: u8) {
        // Simple glow shadow for demonstration
        let shadow_color = Color::new(0, 0, 0);
        let shadow_rect = Rect::new(rect.x + 4, rect.y + 4, rect.width, rect.height);
        self.draw_rounded_rect(shadow_rect, radius, shadow_color);
    }
}

/// 2D Point coordinate
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

/// Generic syscall wrapper for x86_64
#[inline(always)]
pub unsafe fn syscall(num: usize, arg1: usize, arg2: usize, arg3: usize) -> isize {
    let mut ret: isize;
    asm!(
        "int 0x80",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        lateout("rax") ret,
    );
    ret
}

pub fn open(path: &str, flags: usize) -> isize {
    unsafe {
        syscall(SYS_SCHEME_OPEN, path.as_ptr() as usize, path.len(), flags)
    }
}

pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    unsafe {
        syscall(SYS_SCHEME_READ, fd, buf.as_mut_ptr() as usize, buf.len())
    }
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    unsafe {
        syscall(SYS_SCHEME_WRITE, fd, buf.as_ptr() as usize, buf.len())
    }
}

pub fn mmap(addr: usize, len: usize, prot: usize, flags: usize, fd: usize, offset: usize) -> isize {
    // Basic mmap wrapper (simplified for Stage 31.2)
    unsafe {
        syscall(SYS_MMAP, fd, len, offset)
    }
}

pub fn exit(status: usize) -> ! {
    unsafe {
        syscall(SYS_EXIT, status, 0, 0);
    }
    loop {}
}

pub fn spawn(module: &[u8], priority: usize) -> isize {
    unsafe {
        syscall(SYS_SPAWN, module.as_ptr() as usize, module.len(), priority)
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
