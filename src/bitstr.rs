use bit::Bit;
use std::str::Bytes;

pub struct BitStr<'a> {
    bytes: Bytes<'a>,
    cur: Option<u8>,
    cur_idx: u8,
}

impl<'a> BitStr<'a> {
    pub fn from_str(s: &'a str) -> BitStr {
        BitStr {
            bytes: s.bytes(),
            cur: None,
            cur_idx: 0,
        }
    }

    pub fn to_binary(self) -> String {
        self.map(|b| format!("{}", b.as_u8()))
            .collect::<Vec<String>>()
            .join("")
    }
}

impl<'a> Iterator for BitStr<'a> {
    type Item = Bit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.is_none() || self.cur_idx >= 8 {
            self.cur = self.bytes.next();
            self.cur_idx = 0;
        }

        if let Some(byte) = self.cur {
            let bit = Bit::from_u8((byte >> self.cur_idx) & 1);
            self.cur_idx += 1;
            return Some(bit);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate_single() {
        assert_eq!(BitStr::from_str("a").to_binary(), "10000110")
    }

    #[test]
    fn iterate_double() {
        assert_eq!(BitStr::from_str("bc").to_binary(), "0100011011000110")
    }
}
