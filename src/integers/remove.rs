use std::collections::VecDeque;

fn Solve(data: &[i32]) -> Vec<i32> {
    let mut result = Vec::<i32>::new();
    if data.is_empty() {
        return result;
    }

    if data.len() == 1 {
        result.push(data[0]);
        return result;
    }

    let mut ref_data = std::collections::HashMap::<i32, bool>::new();

    for i in (0..=data.len() - 1).rev() {
        if ref_data.contains_key(&data[i]) {
            continue;
        }
        result.push(data[i]);
        ref_data.insert(data[i], true);
    }

    result.reverse();

    result
}

fn Solve_with_deque(data: &[i32]) -> Vec<i32> {
    let mut result = VecDeque::<i32>::new();
    if data.is_empty() {
        return Vec::new();
    }

    if data.len() == 1 {
        return vec![data[0]];
    }

    let mut ref_data = std::collections::HashMap::<i32, bool>::new();

    for i in (0..=data.len() - 1).rev() {
        if ref_data.contains_key(&data[i]) {
            continue;
        }
        result.push_front(data[i]);
        ref_data.insert(data[i], true);
    }

    result.into()
}

#[cfg(test)]
mod working_with_duplicates_tests {
    use crate::integers::remove;

    #[test]
    fn test_solve() {
        // Given
        let cases = std::collections::HashMap::from([(
            "[3, 4, 4, 3, 6, 3]",
            (&[3, 4, 4, 3, 6, 3], &[4, 6, 3]),
        )]);

        for (test_name, test_data) in cases.into_iter() {
            // When
            let got = remove::Solve(test_data.0);
            // Then
            assert_eq!(got, test_data.1, "{}", test_name);
        }
    }

    #[test]
    fn test_solve_deque() {
        // Given
        let cases = std::collections::HashMap::from([(
            "[3, 4, 4, 3, 6, 3]",
            (&[3, 4, 4, 3, 6, 3], &[4, 6, 3]),
        )]);

        for (test_name, test_data) in cases.into_iter() {
            // When
            let got = remove::Solve_with_deque(test_data.0);
            // Then
            assert_eq!(got, test_data.1, "{}", test_name);
        }
    }
}
