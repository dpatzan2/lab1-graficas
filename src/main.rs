mod framebuffer;
mod polygon;

use framebuffer::Framebuffer;
use polygon::fill_polygon;
use raylib::prelude::*;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600, Color::BLACK);
    framebuffer.set_background_color(Color::BLACK);
    framebuffer.clear();

    let poly1 = vec![
        Vector2::new(165.0, 380.0), Vector2::new(185.0, 360.0), Vector2::new(180.0, 330.0), Vector2::new(207.0, 345.0),
        Vector2::new(233.0, 330.0), Vector2::new(230.0, 360.0), Vector2::new(250.0, 380.0), Vector2::new(220.0, 385.0),
        Vector2::new(205.0, 410.0), Vector2::new(193.0, 383.0),
    ];

    let poly2 = vec![
        Vector2::new(321.0, 335.0), Vector2::new(288.0, 286.0), Vector2::new(339.0, 251.0), Vector2::new(374.0, 302.0),
    ];

    let poly3 = vec![
        Vector2::new(377.0, 249.0), Vector2::new(411.0, 197.0), Vector2::new(436.0, 249.0),
    ];

    let poly4 = vec![
        Vector2::new(413.0, 177.0), Vector2::new(448.0, 159.0), Vector2::new(502.0, 88.0), Vector2::new(553.0, 53.0),
        Vector2::new(535.0, 36.0), Vector2::new(676.0, 37.0), Vector2::new(660.0, 52.0), Vector2::new(750.0, 145.0),
        Vector2::new(761.0, 179.0), Vector2::new(672.0, 192.0), Vector2::new(659.0, 214.0), Vector2::new(615.0, 214.0),
        Vector2::new(632.0, 230.0), Vector2::new(580.0, 230.0), Vector2::new(597.0, 215.0), Vector2::new(552.0, 214.0),
        Vector2::new(517.0, 144.0), Vector2::new(466.0, 180.0),
    ];

    let hole_poly5 = vec![
        Vector2::new(682.0, 175.0), Vector2::new(708.0, 120.0), Vector2::new(735.0, 148.0), Vector2::new(739.0, 170.0),
    ];

    framebuffer.set_current_color(Color::ORANGE);
    fill_polygon(&mut framebuffer, &poly1);
    framebuffer.set_current_color(Color::WHITE);
    framebuffer.draw_polygon_outline(&poly1);

    framebuffer.set_current_color(Color::BLUE);
    fill_polygon(&mut framebuffer, &poly2);
    framebuffer.set_current_color(Color::WHITE);
    framebuffer.draw_polygon_outline(&poly2);

    framebuffer.set_current_color(Color::RED);
    fill_polygon(&mut framebuffer, &poly3);
    framebuffer.set_current_color(Color::WHITE);
    framebuffer.draw_polygon_outline(&poly3);

    framebuffer.set_current_color(Color::GREEN);
    fill_polygon(&mut framebuffer, &poly4);
   
    framebuffer.set_current_color(Color::BLACK);
    fill_polygon(&mut framebuffer, &hole_poly5);
    
    
    framebuffer.set_current_color(Color::WHITE);
    framebuffer.draw_polygon_outline(&poly4);
    framebuffer.draw_polygon_outline(&hole_poly5);

    framebuffer.render_to_file("out.png");
    framebuffer.render_to_bmp("out.bmp");
    println!("Imagen generada como out.png y out.bmp");
}
