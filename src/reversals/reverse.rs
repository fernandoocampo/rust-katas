fn reverse_string(value: &str) -> String {
    if value == "" {
        return "".to_string();
    }
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

fn reverse_words(str: &str) -> String {
    let word_iter = str.split(" ");
    let mut result = "".to_string();
    
    for word in word_iter {
        result.push_str(&reverse_string(word));
        result.push_str(" ");
    }

    result.remove(result.len()-1);
    
    result
}

fn reverse_words_clever(str: &str) -> String {
    str.to_string()
        .split(" ")
        .map(|sub| sub.chars().rev().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

fn spin_words(words: &str) -> String {
    words.to_string()
        .split_ascii_whitespace()
        .map(|sub| {
            match sub.len() >= 5 {
                true => sub.chars().rev().collect(),
                false => sub.to_string(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
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

    #[test]
    fn test_reverse_words() {
        // Given
        let cases = HashMap::from([
            ("This is an example!", "sihT si na !elpmaxe"),
            ("double  spaces", "elbuod  secaps"),
        ]);

        for (input, want) in cases.into_iter() {
            // When
            let got = reverse::reverse_words(input);
            // Then
            assert_eq!(want, got.to_string());
        }
    }

    #[test]
    fn test_reverse_words_clever() {
        // Given
        let cases = HashMap::from([
            ("This is an example!", "sihT si na !elpmaxe"),
            ("double  spaces", "elbuod  secaps"),
        ]);

        for (input, want) in cases.into_iter() {
            // When
            let got = reverse::reverse_words_clever(input);
            // Then
            assert_eq!(want, got.to_string());
        }
    }

    #[test]
    fn test_spin_words() {
        // Given
        let cases = HashMap::from([
            ("Hey fellow warriors", "Hey wollef sroirraw"),
            ("This is a test", "This is a test"),
            ("This is another test", "This is rehtona test"),
        ]);

        for (input, want) in cases.into_iter() {
            // When
            let got = reverse::spin_words(input);
            // Then
            assert_eq!(want, got.to_string());
        }
    }
}