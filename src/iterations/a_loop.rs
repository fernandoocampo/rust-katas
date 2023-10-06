fn to_lower_case(data: Vec<String>) -> Vec<String> {
    let mut result = Vec::<String>::new();
    let mut i: usize = 0;
    loop {
        if i == data.len() {
            break;
        }
        result.insert(i, data[i].to_ascii_lowercase());
        i += 1;
    }
    result
}

fn repeat(value: char, repeat_number: usize) -> String {
    let mut result: String = "".to_string();
    let mut i: usize = 1;
    loop {
        if i > repeat_number {
            break;
        }
        result.push(value);
        i += 1;
    }
    result
}

#[cfg(test)]
mod loop_tests {
    use crate::iterations::a_loop;
    #[test]
    fn test_to_lower_case() {
        // Given
        let input = vec!["ALPHA".to_string(), "BRAVO".to_string(), "CHARLIE".to_string(), "DELTA".to_string()];
        let want = vec!["alpha".to_string(), "bravo".to_string(), "charlie".to_string(), "delta".to_string()];
        // When
        let got = a_loop::to_lower_case(input);
        // Then
        assert_eq!(want, got);
    }

    #[test]
    fn test_repeat() {
        // Given
        let char_value = 'c';
        let repeat_number: usize = 8;
        let want = "cccccccc";
        // When
        let got = a_loop::repeat(char_value, repeat_number);
        // Then
        assert_eq!(want, got);
    }
}