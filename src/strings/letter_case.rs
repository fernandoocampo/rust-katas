fn is_upper_case(s: &str) -> bool {
    s == s.to_uppercase()
}

#[cfg(test)]
mod letter_cases_tests {
    #[test]
    fn test_is_upper_case() {
        // Given
        let cases = std::collections::HashMap::from([
            ("c", false),
            ("C", true),
            ("hello I AM DONALD", false),
            ("HELLO I AM DONALD", true),
            ("ACSKLDFJSgSKLDFJSKLDFJ", false),
            ("ACSKLDFJSGSKLDFJSKLDFJ", true),
        ]);

        for (given, want) in cases.into_iter() {
            // When
            let got = crate::strings::letter_case::is_upper_case(given);
            // Then
            assert_eq!(want, got)
        }
    }
}
