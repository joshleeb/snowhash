#![feature(macro_at_most_once_rep)]

extern crate rand;

use bitstr::BitStr;
use point::Point;
use rand::{Rng, SeedableRng, StdRng};

pub mod point;

mod bit;
mod bitstr;

/// Generate points from the provided hash string.
pub fn generate(hash: &str) -> Vec<Point> {
    let seed: &[_] = &[hash_sum(hash)];
    let mut rng: StdRng = SeedableRng::from_seed(seed);

    let mut open = vec![Point::origin()];
    let mut closed = vec![];
    let mut filled = vec![];

    for bit in BitStr::from_str(hash) {
        let index = rng.gen_range(0, open.len());
        let point = open.remove(index);

        if point.on_axis() || bit.as_bool() {
            let mut reflection = point.reflection();
            filled.append(&mut reflection);

            let mut extension = extend(&point, &closed);
            open.append(&mut extension);
        }
        closed.push(point);
    }
    filled
}

/// Sum of the integer value of each character in the provided hash.
fn hash_sum(hash: &str) -> usize {
    hash.as_bytes()
        .into_iter()
        .fold(0, |acc, byte| acc + *byte as usize)
}

fn extend(point: &Point, closed: &Vec<Point>) -> Vec<Point> {
    point
        .neighbours()
        .into_iter()
        .filter(|p| in_slice(p) && !closed.contains(p))
        .collect()
}

fn in_slice(point: &Point) -> bool {
    point.x() >= point.y() && point.y() >= 0
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
