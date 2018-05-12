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

impl Point {
    pub fn slice_neighbours(&self) -> Vec<Point> {
        self.neighbours()
            .into_iter()
            .filter(|p| p.0 >= p.1 && p.1 >= 0)
            .collect()
    }

    pub fn reflection(&self) -> Vec<Self> {
        match *self {
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

    fn neighbours(&self) -> Vec<Point> {
        pts![
            (self.0 + 1, self.1),     // right
            (self.0 - 1, self.1),     // left
            (self.0, self.1 + 1),     // top right
            (self.0, self.1 - 1),     // bottom left
            (self.0 - 1, self.1 + 1), // top right
            (self.0 + 1, self.1 - 1), // bottom left
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbours_at_origin() {
        let point = Point(0, 0);
        assert_eq!(
            point.neighbours(),
            pts![(1, 0), (-1, 0), (0, 1), (0, -1), (-1, 1), (1, -1)]
        )
    }

    #[test]
    fn neighbours_at_point() {
        let point1 = Point(1, 0);
        assert_eq!(
            point1.neighbours(),
            pts![(2, 0), (0, 0), (1, 1), (1, -1), (0, 1), (2, -1)]
        );

        let point2 = Point(2, 1);
        assert_eq!(
            point2.neighbours(),
            pts![(3, 1), (1, 1), (2, 2), (2, 0), (1, 2), (3, 0)]
        );
    }

    #[test]
    fn slice_neighbours_at_origin() {
        let point = Point(0, 0);
        assert_eq!(point.slice_neighbours(), pts![(1, 0)])
    }

    #[test]
    fn slice_neighbours_at_point() {
        let point1 = Point(1, 0);
        assert_eq!(point1.slice_neighbours(), pts![(2, 0), (0, 0), (1, 1)]);

        let point2 = Point(2, 1);
        assert_eq!(
            point2.slice_neighbours(),
            pts![(3, 1), (1, 1), (2, 2), (2, 0), (3, 0)]
        );
    }

    #[test]
    fn reflection_at_origin() {
        let point = Point(0, 0);
        assert_eq!(point.reflection(), pts![(0, 0)])
    }

    #[test]
    fn reflection_at_point_on_x_axis() {
        let point = Point(2, 0);
        assert_eq!(
            point.reflection(),
            pts![(2, 0), (0, 2), (-2, 0), (0, -2), (2, -2), (-2, 2)],
        )
    }

    #[test]
    fn reflection_at_point() {
        let point1 = Point(2, 1);
        assert_eq!(
            point1.reflection(),
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

        let point2 = Point(3, 1);
        assert_eq!(
            point2.reflection(),
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
