fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return input;
    }

    let mut count_positive = 0;
    let mut sum_negative = 0;

    for value in input.into_iter() {
        if value > 0 {
            count_positive += 1;
            continue
        }

        if value < 0 {
            sum_negative += value;
        }
    }

    vec![count_positive, sum_negative]
}

fn count_positives_sum_negatives_two(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return input;
    }

    vec![
        input.iter().filter(|&&x| x > 0).count() as i32, 
        input.iter().filter(|&&x| x < 0).sum()
    ]
}

#[cfg(test)]
mod count_tests {
    use crate::integers::count;
    #[test]
    fn test_count_positives_sum_negatives() {
        // Given
        let cases = std::collections::HashMap::from([
            ("[10, -65]", (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15], vec![10, -65])),
        ]);

        for (test_name, test_data) in cases.into_iter() {
            // When
            let got = count::count_positives_sum_negatives(test_data.0);
            // Then
            assert_eq!(test_data.1, got, "want: {}", test_name);
        }
    }

    #[test]
    fn test_count_positives_sum_negatives_two() {
        // Given
        let cases = std::collections::HashMap::from([
            ("[10, -65]", (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15], vec![10, -65])),
        ]);

        for (test_name, test_data) in cases.into_iter() {
            // When
            let got = count::count_positives_sum_negatives_two(test_data.0);
            // Then
            assert_eq!(test_data.1, got, "want: {}", test_name);
        }
    }
}