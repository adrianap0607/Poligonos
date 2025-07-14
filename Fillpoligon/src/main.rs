mod framebuffer;
use framebuffer::*;
use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 600;

    let mut fb = Framebuffer::new(width, height, Color::BLACK);

    // Polígono 1
    let polygon1 = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];
    fb.fill_polygon_scanline(&polygon1, Color::YELLOW);
    fb.set_current_color(Color::WHITE);
    for i in 0..polygon1.len() {
        let (x0, y0) = polygon1[i];
        let (x1, y1) = polygon1[(i + 1) % polygon1.len()];
        fb.draw_line_bresenham(x0, y0, x1, y1);

    }
  // Guardar resultado
    fb.render_to_file("out.png");
    fb.render_to_file("out.bmp");
}