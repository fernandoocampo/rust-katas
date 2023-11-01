fn between(a: i16, b: i16) -> Vec<i16> {
    let mut result = Vec::<i16>::new();

    for i in a..=b {
        result.push(i);
    }

    result
}

fn between_two(a: i16, b: i16) -> Vec<i16> {
    (a..=b).collect()
}

fn between_three(a: i16, b: i16) -> Vec<i16> {
    Vec::from_iter(a..=b)
}

#[cfg(test)]
mod count_tests {
    use crate::integers::between;
    #[test]
    fn test_between() {
        // Given
        let cases = std::collections::HashMap::from([
            ("1_4", (1, 4, vec![1,2,3,4])),
            ("-2_2", (-2, 2, vec![-2,-1,0,1,2])),
        ]);

        for (test_name, test_data) in cases.into_iter() {
            // When
            let got = between::between(test_data.0, test_data.1);
            // Then
            assert_eq!(test_data.2, got, "want: {}", test_name);
        }
    }
    #[test]
    fn test_between_two() {
        // Given
        let cases = std::collections::HashMap::from([
            ("1_4", (1, 4, vec![1,2,3,4])),
            ("-2_2", (-2, 2, vec![-2,-1,0,1,2])),
        ]);

        for (test_name, test_data) in cases.into_iter() {
            // When
            let got = between::between_two(test_data.0, test_data.1);
            // Then
            assert_eq!(test_data.2, got, "want: {}", test_name);
        }
    }
    #[test]
    fn test_between_three() {
        // Given
        let cases = std::collections::HashMap::from([
            ("1_4", (1, 4, vec![1,2,3,4])),
            ("-2_2", (-2, 2, vec![-2,-1,0,1,2])),
        ]);

        for (test_name, test_data) in cases.into_iter() {
            // When
            let got = between::between_three(test_data.0, test_data.1);
            // Then
            assert_eq!(test_data.2, got, "want: {}", test_name);
        }
    }
}