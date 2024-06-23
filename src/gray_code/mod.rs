fn gray_code(n: u32) -> u32 {
    return n ^ (n >> 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(
            gray_code(0),
            0,
            "Gray code for 0: expected 0, got {}",
            gray_code(0)
        );
        assert_eq!(
            gray_code(1),
            1,
            "Gray code for 1: expected 1, got {}",
            gray_code(1)
        );
        assert_eq!(
            gray_code(2),
            3,
            "Gray code for 2: expected 3, got {}",
            gray_code(2)
        );
        assert_eq!(
            gray_code(3),
            2,
            "Gray code for 3: expected 2, got {}",
            gray_code(3)
        );
        assert_eq!(
            gray_code(4),
            6,
            "Gray code for 4: expected 6, got {}",
            gray_code(4)
        );
        assert_eq!(
            gray_code(5),
            7,
            "Gray code for 5: expected 7, got {}",
            gray_code(5)
        );
        assert_eq!(
            gray_code(6),
            5,
            "Gray code for 6: expected 5, got {}",
            gray_code(6)
        );
        assert_eq!(
            gray_code(7),
            4,
            "Gray code for 7: expected 4, got {}",
            gray_code(7)
        );
        assert_eq!(
            gray_code(8),
            12,
            "Gray code for 8: expected 12, got {}",
            gray_code(8)
        );
    }

    #[test]
    fn test_large_numbers() {
        let input = 2147483648;
        let expected = 3221225472;
        let result = gray_code(input);
        assert_eq!(
            result, expected,
            "Gray code for {}: expected {}, got {}",
            input, expected, result
        );

        let input = 4294967295;
        let expected = 2147483648;
        let result = gray_code(input);
        assert_eq!(
            result, expected,
            "Gray code for {}: expected {}, got {}",
            input, expected, result
        );
    }
}
