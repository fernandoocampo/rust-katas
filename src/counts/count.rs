const VOWELS: &str = "aeiou";

fn count_vowels(value: &str) -> usize {
    value.chars().filter(|c| VOWELS.contains(*c)).count()
}

#[cfg(test)]
mod count_tests {
    use std::collections::HashMap;
    use crate::counts::count::count_vowels;
    #[test]
    fn test_count_vowels() {
        // Given
        let cases = HashMap::from([
            ("hola",2),
            ("anitalavalatina",8),
            ("murcielago",5),
        ]);
        
        for (value, want) in cases.into_iter() {
            // When
            let got = count_vowels(value);
            // Then
            assert_eq!(want, got, "want: {}, but got: {}", want, got)
        }
    }
}