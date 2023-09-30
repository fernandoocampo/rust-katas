fn reverse_string(value: &str) -> String {
    let mut chars = value.chars().collect::<Vec<char>>();
    let mut i: usize = 0;
    let mut j: usize = value.len()-1;
    while i < j {
        (chars[i], chars[j]) = (chars[j], chars[i]);
        i += 1;
        j -= 1;
    }
    
    String::from_iter(chars)
}

#[cfg(test)]
mod reverse_stes {
    use std::collections::HashMap;
    use crate::reversals::reverse;

    #[test]
    fn test_reverse_string() {
        // Given
        let cases = HashMap::from([
            ("hola","aloh"),
            ("anitalavalatina","anitalavalatina"),
        ]);
        
        for (value, want) in cases.into_iter() {
            // When
            let got = reverse::reverse_string(value);
            // Then
            assert_eq!(want, got, "want: {}, but got: {}", want, got)
        }
    }

    #[test]
    fn test_reverse_string_native() {
        // Given
        let cases = HashMap::from([
            ("hola","aloh"),
            ("anitalavalatina","anitalavalatina"),
        ]);
        
        for (value, want) in cases.into_iter() {
            // When
            let got: String = value.chars().rev().collect();
            // Then
            assert_eq!(want, got, "want: {}, but got: {}", want, got)
        }
    }
}