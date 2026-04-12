#![no_std]
#![no_main]

extern crate alloc;

use libaether::{open, mmap, exit, Color, Rect, Point};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 1. Open the Sovereign Display (Orbital Bridge)
    let display_fd = open("display:main", 0);
    if display_fd < 0 {
        exit(1);
    }

    // 2. Map Framebuffer (Sovereign Resource Access)
    let fb_size = 1024 * 768 * 4; // Simulated 1024x768 32bpp
    let fb_ptr = mmap(0, fb_size, 3, 0, display_fd as usize, 0);
    if fb_ptr < 0 {
        exit(2);
    }

    let fb = unsafe { core::slice::from_raw_parts_mut(fb_ptr as *mut u32, 1024 * 768) };

    // 3. [PHASE 35] Render AetherUI Sovereign Dashboard using Renderer
    use libaether::Renderer;
    let mut renderer = Renderer::new(fb, 1024, 768);
    render_dashboard(&mut renderer);

    loop {}
}

fn render_dashboard(renderer: &mut libaether::Renderer) {
    let width = renderer.width;
    let height = renderer.height;

    // Deep Space Gradient Background
    let color_start = Color::from_hex(0x0A0F2E); // Cyber Blue
    let color_end = Color::from_hex(0x1A237E);   // Royal Indigo
    
    for y in 0..height {
        let t = (y * 255 / height) as u8;
        let color = Color::lerp(color_start, color_end, t);
        let c_val = 0xFF000000 | ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32);
        for x in 0..width {
            renderer.fb[y * width + x] = c_val;
        }
    }

    // [SOVEREIGN CARDS] Using Rounded Rects and Shadows
    let main_panel = Rect::new(50, 50, 400, 300);
    renderer.draw_shadow(main_panel, 12, 128);
    renderer.draw_rounded_rect(main_panel, 12, Color::from_hex(0x1565C0));

    let status_card = Rect::new(480, 50, 300, 120);
    renderer.draw_shadow(status_card, 8, 100);
    renderer.draw_rounded_rect(status_card, 8, Color::from_hex(0x283593));
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
