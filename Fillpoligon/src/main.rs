mod framebuffer;
use framebuffer::*;
use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 600;

    let mut fb = Framebuffer::new(width, height, Color::BLACK);

     let polygon3 = vec![
        (377, 249), (411, 197), (436, 249),
    ];
    fb.fill_polygon_scanline(&polygon3, Color::RED);
    fb.set_current_color(Color::WHITE);
    for i in 0..polygon3.len() {
        let (x0, y0) = polygon3[i];
        let (x1, y1) = polygon3[(i + 1) % polygon3.len()];
        fb.draw_line_bresenham(x0, y0, x1, y1);
    }


  // Guardar resultado
    fb.render_to_file("out.png");
    fb.render_to_file("out.bmp");
}