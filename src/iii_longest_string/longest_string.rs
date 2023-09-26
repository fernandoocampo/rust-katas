fn length_of_longest_substring(s: String) -> i32 {
    if s.len() < 2 {
        return s.len() as i32;
    }

    let mut max: usize = 0;
    let mut remainder = s.clone();

    for char_value in s.chars() {
        if remainder.len() == 1 || remainder.len() <= max {
            break;
        }

        remainder = remainder[1..].to_string();

        let mut new_word: String = "".to_string();

        new_word.push(char_value);
        let mut count = 1;

        for nl in remainder.chars() {
            if new_word.contains(nl) {
                break;
            }
            new_word.push(nl);
            count += 1;
        }

        if count > max {
            max = count;
        }

        new_word.clear();
    }

    max as i32
}

fn length_of_longest_substring_i(s: String) -> i32 {
    let mut max_len: usize = 0;

    // [1] longest substring is the one with the largest
    //    difference between positions of repeated characters;
    //    thus, we should create a storage for such positions
    let mut pos: [usize; 128] = [0; 128];

    // [2] while iterating through the string (i.e., moving
    //    the end of the sliding window), we should also
    //    update the start of the window
    let mut start: usize = 0;

    for (end, ch) in s.chars().enumerate() {
        // [3] get the position for the start of sliding window
        //    with no other occurences of 'ch' in it
        start = start.max(pos[ch as usize]);

        // [4] update maximum length
        max_len = max_len.max(end - start + 1);

        // [5] set the position to be used in [3] on next iterations
        pos[ch as usize] = end + 1;
    }

    return max_len as i32;
}

pub fn length_of_longest_substring_ii(s: String) -> i32 {
    use std::collections::HashMap;
    let mut hm = HashMap::new();
    let mut start = 0;
    let mut res = 0;
    
    for (idx, ch) in s.chars().enumerate() {
        start = start.max(hm.insert(ch, idx as i32).unwrap_or(-1) + 1);
        res = res.max(idx as i32 - start + 1); 
    }
    res
}

#[cfg(test)]
pub mod longest_string_tests {
    use crate::iii_longest_string::longest_string;
    use std::collections::HashMap;

    #[test]
    fn test_length_of_longest_substring() {
        // Given
        let test_cases = HashMap::from([("abcabcbb", 3), ("bbbbb", 1), ("pwwkew", 3)]);

        // When
        for (test_case, want) in test_cases.into_iter() {
            let got = longest_string::length_of_longest_substring(test_case.to_string());
            // Then
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_length_of_longest_substring_i() {
        // Given
        let test_cases = HashMap::from([("abcabcbb", 3), ("bbbbb", 1), ("pwwkew", 3)]);

        // When
        for (test_case, want) in test_cases.into_iter() {
            let got = longest_string::length_of_longest_substring_i(test_case.to_string());
            // Then
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_length_of_longest_substring_ii() {
        // Given
        let test_cases = HashMap::from([("abcabcbb", 3), ("bbbbb", 1), ("pwwkew", 3)]);

        // When
        for (test_case, want) in test_cases.into_iter() {
            let got = longest_string::length_of_longest_substring_ii(test_case.to_string());
            // Then
            assert_eq!(want, got);
        }
    }
}
