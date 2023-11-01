fn sum_i32(i: i32, j: i32) -> i32 {
    i+j
}

fn sum_i8(i: i8, j: i8) -> i8 {
    i+j
}

fn positive_sum(slice: &[i32]) -> i32 {
    slice.iter().filter(|&&x| x > 0).sum()
}

#[cfg(test)]
mod sum_tests {
    use crate::integers::sum;
    #[test]
    fn test_sum_i32() {
        // Given
        let i: i32 = i32::MAX-1;
        let j: i32 = 1;
        let want: i32 = std::i32::MAX;
        // When
        let got = sum::sum_i32(i, j);
        // Then
        assert_eq!(want, got);
    }
    #[test]
    fn test_sum_i8() {
        // Given
        let i: i8 = i8::MAX-2;
        let j: i8 = 2;
        let want: i8 = i8::MAX;
        // When
        let got = sum::sum_i8(i, j);
        // Then
        assert_eq!(want, got);
    }

    #[test]
    fn test_positive_sum() {
        // Given
        let cases = std::collections::HashMap::from([
            ("20_result", (&[1,-4,7,12], 20)),
        ]);

        for (test_name, test_data) in cases.into_iter() {
            // When
            let got = sum::positive_sum(test_data.0);
            // Then
            assert_eq!(test_data.1, got, "want: {}", test_name);
        }
    }
}