use raylib::prelude::*;
use crate::framebuffer::Framebuffer;


pub fn fill_polygon(framebuffer: &mut Framebuffer, points: &[Vector2]) {
    let height = framebuffer.height() as i32;

    for y in 0..height {
        let mut intersections = vec![];

        for i in 0..points.len() {
            let v0 = points[i];
            let v1 = points[(i + 1) % points.len()];

            let y0 = v0.y;
            let y1 = v1.y;
            let y_float = y as f32;

            if (y0 <= y_float && y1 > y_float) || (y1 <= y_float && y0 > y_float) {
                let x0 = v0.x;
                let x1 = v1.x;
                let dx = x1 - x0;
                let dy = y1 - y0;
                if dy != 0.0 {
                    let x = x0 + ((y_float - y0) * dx) / dy;
                    intersections.push(x.round() as i32);
                }
            }
        }

        intersections.sort();

        for pair in intersections.chunks(2) {
            if pair.len() == 2 {
                let x_start = pair[0];
                let x_end = pair[1];
                for x in x_start..=x_end {
                    if x >= 0 && x < framebuffer.width() as i32 {
                        framebuffer.set_pixel(x as u32, y as u32);
                    }
                }
            }
        }
    }
}
