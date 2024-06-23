pub fn adder(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    return adder(a ^ b, (a & b) << 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(adder(1, 2), 3);
        assert_eq!(adder(1, 1), 2);
        assert_eq!(adder(2, 2), 4);
        assert_eq!(adder(5, 2), 7);
    }

    #[test]
    fn test_zero() {
        assert_eq!(adder(0, 0), 0);
        assert_eq!(adder(0, 1), 1);
        assert_eq!(adder(1, 0), 1);
    }

    #[test]
    fn test_commute() {
        assert_eq!(adder(3, 4), adder(4, 3));
        assert_eq!(adder(123, 456), adder(456, 123));
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(adder(1_000_000_000, 2_000_000_000), 3_000_000_000);
        assert_eq!(adder(u32::MAX, 0), u32::MAX);
        assert_eq!(adder(u32::MAX, 1), 0);
    }

    #[test]
    fn test_boundary_values() {
        assert_eq!(adder(u32::MAX, u32::MAX), u32::MAX - 1);
        assert_eq!(adder(u32::MAX / 2, u32::MAX / 2), u32::MAX - 1);
    }
}
