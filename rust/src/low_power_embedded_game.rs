pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.step_by(2)
}

pub struct Position(pub i16, pub i16);
impl Position {
    // from the origin Position(0,0)
    pub fn manhattan(&self) -> i16 {
        let Position(x, y) = self;
        x.abs() + y.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divmod() {
        assert_eq!(divmod(10, 3), (3, 1))
    }

    #[test]
    fn test_evens_ints() {
        let mut even_ints = evens(0_u8..);
        assert_eq!(even_ints.next(), Some(0));
    }

    #[test]
    fn test_evens_from_odds() {
        let mut evens_from_odds = evens(1_i16..);
        assert_eq!(evens_from_odds.next(), Some(1));
    }

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(Position(3, -4).manhattan(), 7)
    }
}
