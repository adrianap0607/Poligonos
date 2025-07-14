mod framebuffer;
use framebuffer::*;
use raylib::prelude::*;

fn main() {
    let width = 800;
    let height = 600;

    let mut fb = Framebuffer::new(width, height, Color::BLACK);

     // Polígono 2
    let polygon2 = vec![
        (321, 335), (288, 286), (339, 251), (374, 302),
    ];
    fb.fill_polygon_scanline(&polygon2, Color::BLUE);
    fb.set_current_color(Color::WHITE);
    for i in 0..polygon2.len() {
        let (x0, y0) = polygon2[i];
        let (x1, y1) = polygon2[(i + 1) % polygon2.len()];
        fb.draw_line_bresenham(x0, y0, x1, y1);
    }


  // Guardar resultado
    fb.render_to_file("out.png");
    fb.render_to_file("out.bmp");
}