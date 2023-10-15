fn even_or_odd(number: i32) -> &'static str {
    if number % 2 == 0 {
        return "Even";
    }

    "Odd"
}

fn even_or_odd_smart(number: i32) -> &'static str {
    match number % 2 {
        0 => "Even",
        _ => "Odd",
    }
}

#[cfg(test)]
mod odd_even_tests {
    use std::collections::HashMap;

    use crate::integers::odd_even;
    #[test]
    fn test_odd_or_even() {
        // Given
        let cases = HashMap::from([
            (1, "Odd"),
            (2, "Even"),
            (3, "Odd"),
            (4, "Even"), // when no 'x' and 'o' is present should return true,
            (5, "Odd"),
        ]);

        for (input, want) in cases.into_iter() {
            // When
            let got = odd_even::even_or_odd(input);
            // Then
            assert_eq!(want, got, "value: {}", input);
        }
    }

    #[test]
    fn test_odd_or_even_smart() {
        // Given
        let cases = HashMap::from([
            (1, "Odd"),
            (2, "Even"),
            (3, "Odd"),
            (4, "Even"), // when no 'x' and 'o' is present should return true,
            (5, "Odd"),
        ]);

        for (input, want) in cases.into_iter() {
            // When
            let got = odd_even::even_or_odd_smart(input);
            // Then
            assert_eq!(want, got, "value: {}", input);
        }
    }
}
