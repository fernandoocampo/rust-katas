fn xo(string: &'static str) -> bool {
    let string_chars = string.to_lowercase();
    let x_count = string_chars.chars()
        .into_iter()
        .filter(|c| *c == 'x')
        .count();

    let o_count = string_chars.chars()
        .into_iter()
        .filter(|c| *c == 'o')
        .count();

    o_count == x_count
}

fn xo_smart(string: &'static str) -> bool {
    let string_chars = string.to_lowercase();


    string_chars.matches("x").count() == string_chars.matches("o").count()
}

#[cfg(test)]
mod counter_tests {
    use std::collections::HashMap;

    use crate::counters::counter;

    #[test]
    fn test_exes_and_ohs() {
        // Given
        let cases = HashMap::from([
            ("ooxx", true),
            ("xooxx", false),
            ("ooxXm", true),
            ("zpzpzpp", true), // when no 'x' and 'o' is present should return true,
            ("zzoo", false),
        ]);

        for (input, want) in cases.into_iter() {
            // When
            let got = counter::xo(input);
            // Then
            assert_eq!(want, got, "value: {}", input);
        }
    }

    #[test]
    fn test_exes_and_ohs_smart() {
        // Given
        let cases = HashMap::from([
            ("ooxx", true),
            ("xooxx", false),
            ("ooxXm", true),
            ("zpzpzpp", true), // when no 'x' and 'o' is present should return true,
            ("zzoo", false),
        ]);

        for (input, want) in cases.into_iter() {
            // When
            let got = counter::xo_smart(input);
            // Then
            assert_eq!(want, got, "value: {}", input);
        }
    }
}