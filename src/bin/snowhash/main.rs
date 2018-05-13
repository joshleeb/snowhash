extern crate cairo;
extern crate snowhash;

use cairo::{Context, Format, ImageSurface};
use snowhash::point::Point;
use std::fs::File;

const IMAGE_SIZE: i32 = 500;
const SCALE: f64 = 3f64;

fn draw_points(ctx: &Context, points: &Vec<Point>) {
    ctx.set_line_width(1.0);
    ctx.set_source_rgb(0.0, 0.0, 0.0);

    for point in points {
        // Convert offset hexagonal coordinates into cartesian coordinates.
        let crt_x =
            SCALE * (3f64.sqrt() * point.x() as f64 + 3f64.sqrt() / 2f64 * point.y() as f64);
        let crt_y = SCALE * (3f64 / 2f64 * point.y() as f64);

        let offset = (IMAGE_SIZE as f64 - SCALE) / 2f64;
        draw_hexagon(ctx, crt_x + offset, crt_y + offset, SCALE);
    }

    ctx.fill();
}

fn draw_hexagon(ctx: &Context, x: f64, y: f64, r: f64) {
    let a = -3f64.sqrt() / 2f64 * r;
    let b = r / 2f64;

    // Rotate hexagon 30 degrees.
    ctx.move_to(a + x, y - b);
    ctx.line_to(x, y - r);
    ctx.line_to(x - a, y - b);
    ctx.line_to(x - a, b + y);
    ctx.line_to(x, r + y);
    ctx.line_to(a + x, b + y);
    ctx.line_to(a + x, y - b);
}

fn main() {
    let surface = ImageSurface::create(Format::ARgb32, IMAGE_SIZE, IMAGE_SIZE).unwrap();
    let ctx = Context::new(&surface);

    let points = snowhash::generate("1f83a1a1cdfa28cb1eb4e851e87d");
    draw_points(&ctx, &points);

    let mut file = File::create("file.png").unwrap();
    surface.write_to_png(&mut file).unwrap();
}
