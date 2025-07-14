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

    // Polígono 3
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

    // Polígono 4 
    let polygon4 = vec![
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37),
        (660, 52), (750, 145), (761, 179), (672, 192), (659, 214), (615, 214),
        (632, 230), (580, 230), (597, 215), (552, 214), (517, 144), (466, 180),
    ];

    // Polígono 5 (agujero dentro del 4)
    let polygon5 = vec![
        (682, 175), (708, 120), (735, 148), (739, 170),
    ];

    fb.fill_polygon_scanline(&polygon4, Color::GREEN);
    fb.fill_polygon_scanline(&polygon5, Color::BLACK);

    fb.set_current_color(Color::WHITE);
    for i in 0..polygon4.len() {
        let (x0, y0) = polygon4[i];
        let (x1, y1) = polygon4[(i + 1) % polygon4.len()];
        fb.draw_line_bresenham(x0, y0, x1, y1);
    }

    for i in 0..polygon5.len() {
        let (x0, y0) = polygon5[i];
        let (x1, y1) = polygon5[(i + 1) % polygon5.len()];
        fb.draw_line_bresenham(x0, y0, x1, y1);
    }

    // Guardar resultado
    fb.render_to_file("out.png");
    fb.render_to_file("out.bmp");

}