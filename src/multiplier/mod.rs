use crate::adder;

pub fn multiplier(a: u32, b: u32) -> u32 {
    fn rec(a: u32, b: u32, reg: u32) -> u32 {
        if b == 0 {
            return reg;
        }
        let reg = if b & 1 != 0 {
            adder::adder(reg, a)
        } else {
            reg
        };
        rec(a << 1, b >> 1, reg)
    }
    return rec(a, b, 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(multiplier(1, 2), 2);
        assert_eq!(multiplier(1, 1), 1);
        assert_eq!(multiplier(2, 2), 4);
        assert_eq!(multiplier(5, 2), 10);
    }

    #[test]
    fn test_zero() {
        assert_eq!(multiplier(0, 0), 0);
        assert_eq!(multiplier(0, 1), 0);
        assert_eq!(multiplier(1, 0), 0);
    }

    #[test]
    fn test_commute() {
        assert_eq!(multiplier(3, 4), multiplier(4, 3));
        assert_eq!(multiplier(123, 456), multiplier(456, 123));
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(multiplier(u32::MAX, 0), 0);
        assert_eq!(multiplier(u32::MAX, 1), u32::MAX);
        assert_eq!(multiplier(2_000, 2_000), 4_000_000);
    }

    #[test]
    fn test_boundary_values() {
        assert_eq!(multiplier(u32::MAX, 1), u32::MAX);
        assert_eq!(multiplier(u32::MAX / 2, 2), u32::MAX - 1);
    }
}
