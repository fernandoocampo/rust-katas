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

fn fake_bin(s: &str) -> String {
    s.chars()
        .map(|f| if f < '5' { '0' } else { '1' } )
        .collect()
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

    #[test]
    fn test_fake_bin() {
        // Given
        let cases = std::collections::HashMap::from([
            ("45385593107843568", ("45385593107843568", "01011110001100111")),
            ("509321967506747", ("509321967506747", "101000111101101")),
            ("366058562030849490134388085", ("366058562030849490134388085", "011011110000101010000011011")),
            ("15889923", ("15889923", "01111100")),
            ("800857237867", ("800857237867", "100111001111")),
        ]);

        for (test_name, test_data) in cases.into_iter() {
            // When
            let got = between::fake_bin(test_data.0);
            // Then
            assert_eq!(test_data.1, got, "{}", test_name);
        }
    }
}