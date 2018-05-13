#![feature(macro_at_most_once_rep)]

extern crate rand;

use bitstr::BitStr;
use point::Point;
use rand::{Rng, SeedableRng, StdRng};

mod bit;
mod bitstr;
mod point;

pub fn snowhash(hash: &str) -> Vec<Point> {
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    let mut frontier = vec![Point::origin()];
    let mut unfilled = vec![];
    let mut filled = vec![];

    for bit in BitStr::from_str(hash) {
        let index = rng.gen_range(0, frontier.len());
        let point = frontier.remove(index);

        let mut untouched_neighbours: Vec<Point> = point
            .neighbours()
            .into_iter()
            .filter(|p| in_slice(p) && !filled.contains(p) && !unfilled.contains(p))
            .collect();
        frontier.append(&mut untouched_neighbours);

        if point.on_axis() || bit.as_bool() {
            let mut reflection = point.reflection();
            filled.append(&mut reflection);
        } else {
            unfilled.push(point)
        }
    }
    filled
}

fn in_slice(point: &Point) -> bool {
    let (x, y) = point.get();
    x >= y && y >= 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_sice_origin() {
        assert!(in_slice(&Point::origin()))
    }

    #[test]
    fn in_sice() {
        assert!(in_slice(&Point::new(1, 0)));
        assert!(!in_slice(&Point::new(0, 1)));
        assert!(!in_slice(&Point::new(2, -1)));
    }
}
