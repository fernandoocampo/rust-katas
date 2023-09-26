fn longest_palindrome(s: String) -> String {
    if is_palindromic(s.as_str()) {
        return s;
    }

    let mut longest = "";
    let (mut i , mut j ): (usize, usize) = (0, 0);

    while j < s.len() {
        if i == s.len()-1 || i >= s.len()-j {
            j += 1;
            i = 0;
            continue;
        }
        let nw = &s[i..s.len()-j];
        if nw.len() < longest.len() || !is_palindromic(nw) {
            i += 1;
            continue;
        }
        if nw.len() > longest.len() {
            longest = nw;
        }
        i += 1;
    }

    if s.len() > 1 && longest.len() == 1 {
        return s[0..1].to_string();
    }

    if longest.is_empty() {
        return s[0..1].to_string();
    }
    
    longest.to_string()
}

fn longest_palindrome_rec(s: String) -> String {
    if is_palindromic(s.as_str()) {
        return s.to_string();
    }

    let result = iterate_word(s.as_str(), "");

    result.to_string()
}

// https://glenngillen.com/learning-rust/substrings/
fn is_palindromic(s: &str) -> bool {
    if s == "" {
        return false;
    }

    if s.len() == 1 {
        return true;
    }

    if s.len() <= 3 {
        return &s[0..1] == &s[s.len()-1..s.len()];
    }


    (&s[0..1] == &s[s.len()-1..s.len()]) && is_palindromic(&s[1..s.len()-1])
}

fn iterate_word<'a>(s: &'a str, mut longest: &'a str) -> &'a str {
    if s.len() == 1 || s.len() <= longest.len() {
        return longest;
    }

    let result = find_longest_palindrome(s, "");
    
    if result.len() > longest.len() {
        longest = result;
    }

    iterate_word(&s[0..s.len()-1], longest)
}

fn find_longest_palindrome<'a>(s: &'a str, mut longest: &'a str) -> &'a str {
    if s == "" || s.len() <= longest.len() {
        return longest;
    }

    if is_palindromic(s) && s.len() > longest.len() {
        longest = s;
    }

    find_longest_palindrome(&s[1..s.len()], longest)
}

#[cfg(test)]
mod palincromic_tests {
    use std::collections::HashMap;
    use crate::v_palindromic::palindromic::is_palindromic;
    use crate::v_palindromic::palindromic::longest_palindrome;
    use crate::v_palindromic::palindromic::longest_palindrome_rec;

    #[test]
    fn test_is_palindromic() {
        // Given
        let test_cases = HashMap::from([
            ("dabad", true),
            ("anitalavalatina", true),
        ]);

        for (input, want) in test_cases.into_iter() {
            // When
            let got = is_palindromic(input);
            // Then
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_longest_palindrome() {
        // Given
        let test_cases = HashMap::from([
            ("babad", "aba"),
            ("cbbd", "bb"),
            ("aacabdkacaa", "aca"),
            ("bb","bb"),
            ("ac","a"),
            ("a","a"),
            ("abcda", "a")
        ]);

        for (input, want) in test_cases.into_iter() {
            // When
            let got = longest_palindrome(input.to_string());
            // Then
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_longest_palindrome_rec() {
        // Given
        let test_cases = HashMap::from([
            ("babad", "aba"),
            ("cbbd", "bb"),
            ("aacabdkacaa", "aca"),
            ("bb","bb"),
            ("ac","c"),
            ("a","a"),
            ("abcda", "a")
        ]);

        for (input, want) in test_cases.into_iter() {
            // When
            let got = longest_palindrome_rec(input.to_string());
            // Then
            assert_eq!(want, got, "given {}", input);
        }
    }
}