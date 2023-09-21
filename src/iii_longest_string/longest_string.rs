fn length_of_longest_substring(s: String) -> i32 {
    3
}

#[cfg(test)]
pub mod longest_string_tests {
    use crate::iii_longest_string::longest_string::length_of_longest_substring;
    use std::collections::HashMap;

    #[test]
    fn test_length_of_longest_substring() {
        // Given
        let test_cases = HashMap::from([("abcabcbb", 3)]);

        // When
        for (test_case, want) in test_cases.into_iter() {
            let got = length_of_longest_substring(test_case.to_string());
            // Then
            assert_eq!(want, got);
        }
    }
}
