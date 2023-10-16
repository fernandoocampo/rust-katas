/// You are going to be given a word. Your job is
/// to return the middle character of the word. If
/// the word's length is odd, return the middle
/// character. If the word's length is even, return
/// the middle 2 characters.
fn get_middle(s: &str) -> &str {
    if s.len() < 3 {
        return s;
    }
    if s.len() % 2 == 0 {
        let index = (s.len() / 2) - 1;
        return s.get(index..index + 2).unwrap();
    }

    let index = (s.len() + 1) / 2;

    println!("index: {}", index);

    s.get(index - 1..index).unwrap()
}

fn get_middle_smart(s: &str) -> &str {
    let len = s.len();
    let from = (len - 1) / 2;
    let to = (len / 2) + 1;
    &s[from..to]
}

#[cfg(test)]
mod substring_tests {
    use crate::strings::substrings;
    use std::collections::HashMap;

    #[test]
    fn test_get_middle() {
        // Given
        let cases = HashMap::from([
            ("test", "es"),
            ("testing", "t"),
            ("middle", "dd"),
            ("A", "A"),
        ]);

        for (input, want) in cases.into_iter() {
            // When
            let got = substrings::get_middle(input);
            // Then
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_get_middle_smart() {
        // Given
        let cases = HashMap::from([
            ("test", "es"),
            ("testing", "t"),
            ("middle", "dd"),
            ("A", "A"),
        ]);

        for (input, want) in cases.into_iter() {
            // When
            let got = substrings::get_middle_smart(input);
            // Then
            assert_eq!(want, got);
        }
    }
}
