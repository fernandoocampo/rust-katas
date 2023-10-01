fn to_upper_case(values: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::with_capacity(values.len());

    let mut index: usize = 0;

    while index < values.len() {
        result.push(values[index].to_ascii_lowercase());
        index += 1;
    }

    result
}

fn repeat(value: char, repeat_number: i8) -> String {
    let mut result = String::new();

    let mut i = 0;
    
    while i < repeat_number {
        result.push(value);
        i += 1;
    }
    
    result
}

#[cfg(test)]
mod sum_tests {
    use crate::iterations::while_loop;
    #[test]
    fn test_sum_i32() {
        // Given
        let input = vec!["ALPHA".to_string(), "BRAVO".to_string(), "CHARLIE".to_string(), "DELTA".to_string()];
        let want = vec!["alpha".to_string(), "bravo".to_string(), "charlie".to_string(), "delta".to_string()];
        // When
        let got = while_loop::to_upper_case(input);
        // Then
        assert_eq!(want, got);
    }

    #[test]
    fn test_repeat() {
        // Given
        let char_value = 'c';
        let repeat_number: i8 = 8;
        let want = "cccccccc";
        // When
        let got = while_loop::repeat(char_value, repeat_number);
        // Then
        assert_eq!(want, got);
    }
}