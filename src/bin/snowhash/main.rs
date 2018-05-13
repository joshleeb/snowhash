extern crate cairo;
extern crate snowhash;

use cairo::{Context, Format, ImageSurface};
use snowhash::point::Point;
use std::fs::File;

fn draw_points(surface: &ImageSurface, points: &Vec<Point>) {
    let ctx = Context::new(&surface);

    ctx.set_line_width(1.0);
    ctx.set_source_rgb(0.0, 0.0, 0.0);

    for point in points {
        ctx.rectangle(
            (point.x() * 5) as f64 + 175.0,
            (point.y() * 5) as f64 + 175.0,
            5.0,
            5.0,
        );
    }

    ctx.fill();
    ctx.stroke();
}

fn main() {
    let surface = ImageSurface::create(Format::ARgb32, 400, 400).unwrap();
    let points = snowhash::generate("1f83a1a1cdfa28cb1eb42a41078be4080051e87d");
    draw_points(&surface, &points);

    let mut file = File::create("file.png").unwrap();
    surface.write_to_png(&mut file).unwrap();
}
