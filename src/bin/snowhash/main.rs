extern crate cairo;
extern crate snowhash;

use cairo::{Context, Format, ImageSurface};
use snowhash::point::Point;
use std::fs::File;

const IMAGE_SIZE: i32 = 1200;
const SCALE: f64 = 10.0;

/// Draw the list of points as hexagons.
fn draw_points(ctx: &Context, points: &Vec<Point>) {
    for point in points {
        let offset = (IMAGE_SIZE as f64 - SCALE) / 2.0;
        let (x, y) = hex_to_cartesian(point.x().into(), point.y().into());
        draw_hexagon(ctx, x + offset, y + offset, SCALE);
    }
    ctx.fill();
}

/// Convert Hex coordinates to Cartesian coordinates.
///
/// From: https://www.redblobgames.com/grids/hexagons/#hex-to-pixel.
fn hex_to_cartesian(hex_x: f64, hex_y: f64) -> (f64, f64) {
    (
        SCALE * (3f64.sqrt() * hex_x + 3f64.sqrt() / 2.0 * hex_y),
        SCALE * (3.0 / 2.0 * hex_y),
    )
}

/// Draw a outline of a hexagon to fill or stroke. The hexagon is roatated by 30 degrees so that
/// the top of pointed instead of flat.
///
/// Deriving these formulas were done with the help from:
/// https://www.mathopenref.com/coordpolycalc.html.
fn draw_hexagon(ctx: &Context, x: f64, y: f64, r: f64) {
    let a = -3f64.sqrt() / 2.0 * r;
    let b = r / 2.0;

    // Rotate hexagon 30 degrees.
    ctx.move_to(a + x, y - b);
    ctx.line_to(x, y - r);
    ctx.line_to(x - a, y - b);
    ctx.line_to(x - a, b + y);
    ctx.line_to(x, r + y);
    ctx.line_to(a + x, b + y);
    ctx.line_to(a + x, y - b);
}

// TODO(joshleeb): Expose CLI.
fn main() {
    let surface = ImageSurface::create(Format::ARgb32, IMAGE_SIZE, IMAGE_SIZE).unwrap();
    let ctx = Context::new(&surface);

    let points = snowhash::generate("cdc1a1a98b37c828f72b2df5550d658c6f092848");
    draw_points(&ctx, &points);

    let mut file = File::create("file.png").unwrap();
    surface.write_to_png(&mut file).unwrap();
}
