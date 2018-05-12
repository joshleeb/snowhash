#![feature(macro_at_most_once_rep)]

macro_rules! pts {
    ($(($x:expr, $y:expr)),*$(,)?) => {
        vec![$(Point($x, $y)),*]
    }
}

#[derive(Debug, Clone)]
pub struct Point(i32, i32);

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl Eq for Point {}

fn neighbours(point: &Point) -> Vec<Point> {
    pts![
        (point.0 + 1, point.1),     // right
        (point.0 - 1, point.1),     // left
        (point.0, point.1 + 1),     // top right
        (point.0, point.1 - 1),     // bottom left
        (point.0 - 1, point.1 + 1), // top right
        (point.0 + 1, point.1 - 1), // bottom left
    ]
}

pub fn slice_neighbours(point: &Point) -> Vec<Point> {
    neighbours(point)
        .into_iter()
        .filter(|p| p.0 >= p.1 && p.1 >= 0)
        .collect()
}

pub fn reflect(point: &Point) -> Vec<Point> {
    match *point {
        Point(0, 0) => pts![(0, 0)],
        Point(x, 0) => pts![(x, 0), (0, x), (-x, 0), (0, -x), (x, -x), (-x, x)],
        Point(x, y) => {
            let sum = x + y;
            pts![
                (x, y),
                (-x, -y),
                (y, x),
                (-y, -x),
                (-x, sum),
                (-y, sum),
                (x, -sum),
                (y, -sum),
                (sum, -x),
                (sum, -y),
                (-sum, x),
                (-sum, y),
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbours_at_origin() {
        assert_eq!(
            neighbours(&Point(0, 0)),
            pts![(1, 0), (-1, 0), (0, 1), (0, -1), (-1, 1), (1, -1)]
        )
    }

    #[test]
    fn neighbours_at_point() {
        assert_eq!(
            neighbours(&Point(1, 0)),
            pts![(2, 0), (0, 0), (1, 1), (1, -1), (0, 1), (2, -1)]
        );
        assert_eq!(
            neighbours(&Point(2, 1)),
            pts![(3, 1), (1, 1), (2, 2), (2, 0), (1, 2), (3, 0)]
        );
    }

    #[test]
    fn slice_neighbours_at_origin() {
        assert_eq!(slice_neighbours(&Point(0, 0)), pts![(1, 0)])
    }

    #[test]
    fn slice_neighbours_at_point() {
        assert_eq!(slice_neighbours(&Point(1, 0)), pts![(2, 0), (0, 0), (1, 1)]);
        assert_eq!(
            slice_neighbours(&Point(2, 1)),
            pts![(3, 1), (1, 1), (2, 2), (2, 0), (3, 0)]
        );
    }

    #[test]
    fn reflection_at_origin() {
        assert_eq!(reflect(&Point(0, 0)), pts![(0, 0)])
    }

    #[test]
    fn reflection_at_point_on_x_axis() {
        assert_eq!(
            reflect(&Point(2, 0)),
            pts![(2, 0), (0, 2), (-2, 0), (0, -2), (2, -2), (-2, 2)],
        )
    }

    #[test]
    fn reflection_at_point() {
        assert_eq!(
            reflect(&Point(2, 1)),
            pts![
                (2, 1),
                (-2, -1),
                (1, 2),
                (-1, -2),
                (-2, 3),
                (-1, 3),
                (2, -3),
                (1, -3),
                (3, -2),
                (3, -1),
                (-3, 2),
                (-3, 1),
            ]
        );
        assert_eq!(
            reflect(&Point(3, 1)),
            pts![
                (3, 1),
                (-3, -1),
                (1, 3),
                (-1, -3),
                (-3, 4),
                (-1, 4),
                (3, -4),
                (1, -4),
                (4, -3),
                (4, -1),
                (-4, 3),
                (-4, 1),
            ]
        );
    }
}
