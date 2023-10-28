fn sort_by_length(arr: &[String]) -> Vec<String> {
    let mut result = Vec::from(arr);
    result.sort_by(|a,b| a.len().cmp(&b.len()));
    result
}

fn sort_by_length_two(arr: &[String]) -> Vec<String> {
    let mut vec = arr.to_vec();
    vec.sort_by_cached_key(|s| s.len());
    vec
}

fn sort_by_length_three(arr: &[String]) -> Vec<String> {
    let mut arr = arr.to_vec();
    arr.sort_by_key(String::len);
    arr
}

#[cfg(test)]
mod sorting_string_tests {
    use crate::strings::sort;

    #[test]
    fn test_sort_by_length() {
        // Given
        let cases = std::collections::HashMap::from([
            (
                "telescopes",
                (
                    vec!["Telescopes".to_string(), "Glasses".to_string(), "Eyes".to_string(), "Monocles".to_string()],
                    vec!["Eyes".to_string(), "Glasses".to_string(), "Monocles".to_string(), "Telescopes".to_string()],
                ),
            )
        ]);

        for (test_name, test_data) in cases.into_iter() {
            // When
            let got = sort::sort_by_length(&test_data.0);
            // Then
            assert_eq!(got, test_data.1, "{}", test_name);
        }
    }

    #[test]
    fn test_sort_by_length_two() {
        // Given
        let cases = std::collections::HashMap::from([
            (
                "telescopes",
                (
                    vec!["Telescopes".to_string(), "Glasses".to_string(), "Eyes".to_string(), "Monocles".to_string()],
                    vec!["Eyes".to_string(), "Glasses".to_string(), "Monocles".to_string(), "Telescopes".to_string()],
                ),
            )
        ]);

        for (test_name, test_data) in cases.into_iter() {
            // When
            let got = sort::sort_by_length_two(&test_data.0);
            // Then
            assert_eq!(got, test_data.1, "{}", test_name);
        }
    }

    #[test]
    fn test_sort_by_length_three() {
        // Given
        let cases = std::collections::HashMap::from([
            (
                "telescopes",
                (
                    vec!["Telescopes".to_string(), "Glasses".to_string(), "Eyes".to_string(), "Monocles".to_string()],
                    vec!["Eyes".to_string(), "Glasses".to_string(), "Monocles".to_string(), "Telescopes".to_string()],
                ),
            )
        ]);

        for (test_name, test_data) in cases.into_iter() {
            // When
            let got = sort::sort_by_length_three(&test_data.0);
            // Then
            assert_eq!(got, test_data.1, "{}", test_name);
        }
    }
}