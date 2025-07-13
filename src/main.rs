mod framebuffer;
mod polygon;

use framebuffer::Framebuffer;
use polygon::fill_polygon;
use raylib::prelude::*;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600, Color::WHITE);
    framebuffer.set_background_color(Color::WHITE);
    framebuffer.clear();

    let poly1 = vec![
        Vector2::new(165.0, 380.0), Vector2::new(185.0, 360.0), Vector2::new(180.0, 330.0), Vector2::new(207.0, 345.0),
        Vector2::new(233.0, 330.0), Vector2::new(230.0, 360.0), Vector2::new(250.0, 380.0), Vector2::new(220.0, 385.0),
        Vector2::new(205.0, 410.0), Vector2::new(193.0, 383.0),
    ];

    

    framebuffer.set_current_color(Color::RED);
    fill_polygon(&mut framebuffer, &poly1);

}
