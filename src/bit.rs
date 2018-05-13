#[derive(Debug)]
pub struct Bit(Option<()>);

impl Bit {
    pub fn hi() -> Bit {
        Bit(Some(()))
    }

    pub fn lo() -> Bit {
        Bit(None)
    }

    pub fn from_u8(n: u8) -> Bit {
        match n {
            0 => Bit::lo(),
            _ => Bit::hi(),
        }
    }

    pub fn as_bool(&self) -> bool {
        self.0.is_some()
    }
}

impl PartialEq for Bit {
    fn eq(&self, other: &Bit) -> bool {
        self.as_bool() == other.as_bool()
    }
}
impl Eq for Bit {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hi_bit() {
        assert!(Bit::hi().as_bool())
    }

    #[test]
    fn lo_bit() {
        assert!(!Bit::lo().as_bool())
    }

    #[test]
    fn hi_from_u8() {
        assert_eq!(Bit::hi(), Bit::from_u8(1));
    }

    #[test]
    fn hi_from_u8_other() {
        assert_eq!(Bit::hi(), Bit::from_u8(2));
    }

    #[test]
    fn lo_from_u8() {
        assert_eq!(Bit::lo(), Bit::from_u8(0));
    }
}
