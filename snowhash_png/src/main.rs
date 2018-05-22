#[macro_use]
extern crate clap;

extern crate cairo;
extern crate snowhash;

use cairo::{Context, Format, ImageSurface};
use snowhash::point::Point;
use std::fs::File;

mod app;

const IMAGE_SIZE: i32 = 500;
const SCALE: f64 = 4.0;

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
/// Deriving these formulas were done with the help of:
/// https://www.mathopenref.com/coordpolycalc.html.
fn draw_hexagon(ctx: &Context, x: f64, y: f64, r: f64) {
    let a = -3f64.sqrt() / 2.0 * r;
    let b = r / 2.0;

    ctx.move_to(a + x, y - b);
    ctx.line_to(x, y - r);
    ctx.line_to(x - a, y - b);
    ctx.line_to(x - a, b + y);
    ctx.line_to(x, r + y);
    ctx.line_to(a + x, b + y);
    ctx.line_to(a + x, y - b);
}

fn main() {
    let app = app::create();
    let matches = app.get_matches();

    let hash = matches.value_of("HASH").unwrap();
    let default_output = format!("{}.png", hash);
    let output = matches.value_of("output").unwrap_or(&default_output);

    let surface = ImageSurface::create(Format::ARgb32, IMAGE_SIZE, IMAGE_SIZE).unwrap();
    let ctx = Context::new(&surface);

    let points = snowhash::generate(hash);
    draw_points(&ctx, &points);

    let mut file = File::create(output).unwrap();
    surface.write_to_png(&mut file).unwrap();
}
