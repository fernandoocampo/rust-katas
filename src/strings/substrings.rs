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

fn ends_with(word: &str, ending: &str) -> bool {
    if word == ending {
        return true;
    }

    if ending.len() > word.len() {
        return false;
    }

    &word[word.len()-ending.len()..] == ending
}

fn ends_with_brief(word: &str, ending: &str) -> bool {
    word.ends_with(ending)
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

    #[test]
    fn test_ends_with() {
        // Given
        let cases = HashMap::from([
            ("abc_c", ("abc", "c", true)),
            ("empty", ("", "", true)),
            ("space", (" ", "", true)),
            ("banana", ("banana", "ana", true)),
            ("a_z", ("a", "z", false)),
            ("empty_t", ("", "t", false)),
        ]);

        for (test_name, test_data) in cases.iter() {
            let input = test_data.0;
            let ends = test_data.1;
            let want = test_data.2;
            let got = substrings::ends_with(input, ends);

            assert_eq!(want, got, "test: {}", test_name);
        }
    }

    #[test]
    fn test_ends_with_brief() {
        // Given
        let cases = HashMap::from([
            ("abc_c", ("abc", "c", true)),
            ("empty", ("", "", true)),
            ("space", (" ", "", true)),
            ("banana", ("banana", "ana", true)),
            ("a_z", ("a", "z", false)),
            ("empty_t", ("", "t", false)),
        ]);

        for (test_name, test_data) in cases.iter() {
            let input = test_data.0;
            let ends = test_data.1;
            let want = test_data.2;
            let got = substrings::ends_with_brief(input, ends);

            assert_eq!(want, got, "test: {}", test_name);
        }
    }
}
