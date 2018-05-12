#[derive(Debug, Clone)]
struct Point(i32, i32);

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl Eq for Point {}

fn neighbours(point: &Point) -> Vec<Point> {
    vec![
        Point(point.0 + 1, point.1),     // right
        Point(point.0 - 1, point.1),     // left
        Point(point.0, point.1 + 1),     // top right
        Point(point.0, point.1 - 1),     // bottom left
        Point(point.0 - 1, point.1 + 1), // top right
        Point(point.0 + 1, point.1 - 1), // bottom left
    ]
}

fn segment_neighbours(point: &Point) -> Vec<Point> {
    neighbours(point)
        .into_iter()
        .filter(|p| p.0 >= p.1 && p.1 >= 0)
        .collect()
}

fn reflexive_points(point: &Point) -> Vec<Point> {
    if *point == Point(0, 0) {
        return vec![point.clone()];
    }
    if point.1 == 0 {
        // On the x-axis.
        return vec![
            point.clone(),
            Point(0, point.0),
            Point(-point.0, point.0),
            Point(-point.0, 0),
            Point(0, -point.0),
            Point(point.0, -point.0),
        ];
    }

    let sum = point.0 + point.1;
    vec![
        point.clone(),
        Point(-point.0, -point.1),
        Point(point.1, point.0),
        Point(-point.1, -point.0),
        Point(-point.0, sum),
        Point(-point.1, sum),
        Point(point.0, -sum),
        Point(point.1, -sum),
        Point(sum, -point.0),
        Point(sum, -point.1),
        Point(-sum, point.0),
        Point(-sum, point.1),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbours_at_origin() {
        assert_eq!(
            vec![
                Point(1, 0),
                Point(-1, 0),
                Point(0, 1),
                Point(0, -1),
                Point(-1, 1),
                Point(1, -1),
            ],
            neighbours(&Point(0, 0))
        )
    }

    #[test]
    fn neighbours_at_point() {
        assert_eq!(
            vec![
                Point(2, 0),
                Point(0, 0),
                Point(1, 1),
                Point(1, -1),
                Point(0, 1),
                Point(2, -1),
            ],
            neighbours(&Point(1, 0))
        );
        assert_eq!(
            vec![
                Point(3, 1),
                Point(1, 1),
                Point(2, 2),
                Point(2, 0),
                Point(1, 2),
                Point(3, 0),
            ],
            neighbours(&Point(2, 1))
        );
    }

    #[test]
    fn segment_neighbours_at_origin() {
        assert_eq!(vec![Point(1, 0)], segment_neighbours(&Point(0, 0)))
    }

    #[test]
    fn segment_neighbours_at_point() {
        assert_eq!(
            vec![Point(2, 0), Point(0, 0), Point(1, 1)],
            segment_neighbours(&Point(1, 0))
        );
        assert_eq!(
            vec![
                Point(3, 1),
                Point(1, 1),
                Point(2, 2),
                Point(2, 0),
                Point(3, 0),
            ],
            segment_neighbours(&Point(2, 1))
        );
    }

    #[test]
    fn reflection_at_origin() {
        assert_eq!(vec![Point(0, 0)], reflexive_points(&Point(0, 0)))
    }

    #[test]
    fn reflection_at_point_on_x_axis() {
        assert_eq!(
            vec![
                Point(2, 0),
                Point(0, 2),
                Point(-2, 2),
                Point(-2, 0),
                Point(0, -2),
                Point(2, -2),
            ],
            reflexive_points(&Point(2, 0))
        )
    }

    #[test]
    fn reflection_at_point() {
        assert_eq!(
            vec![
                Point(2, 1),
                Point(-2, -1),
                Point(1, 2),
                Point(-1, -2),
                Point(-2, 3),
                Point(-1, 3),
                Point(2, -3),
                Point(1, -3),
                Point(3, -2),
                Point(3, -1),
                Point(-3, 2),
                Point(-3, 1),
            ],
            reflexive_points(&Point(2, 1))
        );
        assert_eq!(
            vec![
                Point(3, 1),
                Point(-3, -1),
                Point(1, 3),
                Point(-1, -3),
                Point(-3, 4),
                Point(-1, 4),
                Point(3, -4),
                Point(1, -4),
                Point(4, -3),
                Point(4, -1),
                Point(-4, 3),
                Point(-4, 1),
            ],
            reflexive_points(&Point(3, 1))
        );
    }
}
