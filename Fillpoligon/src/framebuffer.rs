use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    pub background_color: Color,
    pub current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.width as i32, self.height as i32, self.background_color);
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            Image::draw_pixel(&mut self.color_buffer, x as i32, y as i32, self.current_color);
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn render_to_file(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }

    pub fn draw_line_bresenham(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        let mut x0 = x0;
        let mut y0 = y0;
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            self.set_pixel(x0 as u32, y0 as u32);

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x0 += sx;
            }
            if e2 < dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn fill_polygon_scanline(&mut self, points: &Vec<(i32, i32)>, fill_color: Color) {
        if points.is_empty() {
            return;
        }
        let min_y = points.iter().map(|&(_, y)| y).min().unwrap();
        let max_y = points.iter().map(|&(_, y)| y).max().unwrap();

        self.set_current_color(fill_color);

        for y in min_y..=max_y {
            let mut intersections = Vec::new();

            for i in 0..points.len() {
                let (x0, y0) = points[i];
                let (x1, y1) = points[(i + 1) % points.len()];

                if (y0 <= y && y1 > y) || (y1 <= y && y0 > y) {
                    if y1 != y0 {
                        let x = x0 + (y - y0) * (x1 - x0) / (y1 - y0);
                        intersections.push(x);
                    }
                }
            }

            intersections.sort();

            let mut i = 0;
            while i + 1 < intersections.len() {
                let x_start = intersections[i];
                let x_end = intersections[i + 1];

                // Rellenar de borde a borde; luego el contorno blanco lo cubrirá
                for x in x_start..=x_end {
                    self.set_pixel(x as u32, y as u32);
                }
                i += 2;
            }
        }
    }
}
