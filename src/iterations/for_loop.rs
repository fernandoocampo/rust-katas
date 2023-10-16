fn to_upper_case(values: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::with_capacity(values.len());

    for value in values.into_iter() {
        result.push(value.to_ascii_lowercase());
    }

    result
}

fn repeat(value: char, repeat_number: i8) -> String {
    let mut result = String::new();
    
    for _i in 1..=repeat_number {
        result.push(value);
    }
    
    result
}

fn find_smallest_int(arr: &[i32]) -> i32 {
    let mut result: i32 = arr[0];

    for value in arr.into_iter() {
        if *value < result {
            result = *value;
        }
    }

    result
}

fn find_smallest_int_smart(arr: &[i32]) -> i32 {
    match arr.iter().min() {
        Some(&v) => v,
        None => 0,
    }
}

#[cfg(test)]
mod sum_tests {
    use std::collections::HashMap;

    use crate::iterations::for_loop;
    #[test]
    fn test_to_upper_case() {
        // Given
        let input = vec!["ALPHA".to_string(), "BRAVO".to_string(), "CHARLIE".to_string(), "DELTA".to_string()];
        let want = vec!["alpha".to_string(), "bravo".to_string(), "charlie".to_string(), "delta".to_string()];
        // When
        let got = for_loop::to_upper_case(input);
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
        let got = for_loop::repeat(char_value, repeat_number);
        // Then
        assert_eq!(want, got);
    }

    #[test]
    fn test_find_smallest_int() {
        // Given
        let cases = HashMap::from([
            (&[34, 15, 88, 2], 2),
            (&[34, -345, -1, 100], -345),
        ]);

        for (input, want) in cases.into_iter() {
            // When
            let got = for_loop::find_smallest_int(input);
            // Then
            assert_eq!(want, got);
        }
    }

    #[test]
    fn test_find_smallest_int_smart() {
        // Given
        let cases = HashMap::from([
            (&[34, 15, 88, 2], 2),
            (&[34, -345, -1, 100], -345),
        ]);

        for (input, want) in cases.into_iter() {
            // When
            let got = for_loop::find_smallest_int_smart(input);
            // Then
            assert_eq!(want, got);
        }
    }
}