use itertools::Itertools;

fn even_or_odd(number: i32) -> &'static str {
    if number % 2 == 0 {
        return "Even";
    }

    "Odd"
}

fn even_or_odd_smart(number: i32) -> &'static str {
    match number % 2 {
        0 => "Even",
        _ => "Odd",
    }
}

fn sort_odd_array(arr: &[i32]) -> Vec<i32> {
    let mut indexes: Vec<usize> = Vec::<usize>::new();
    let mut odds: Vec<i32> = Vec::<i32>::new();

    for i in 0..arr.len() {
        if arr[i]%2 != 0 {
            indexes.push(i);
            odds.push(arr[i]);
        }
    }

	if odds.len() == 0 {
		return arr.to_vec();
	}

    odds.sort();

    let mut result = arr.to_vec();

	for i in 0..indexes.len() {
		result[indexes[i]] = odds[i]
	}

	result.to_vec()
}

fn sort_odd_array_smart(xs: &[i32]) -> Vec<i32> {
    let mut os = xs.iter().filter(|&x| x % 2 != 0).sorted();
    xs.iter().map(|x| if x % 2 != 0 { os.next().unwrap() } else { x }).cloned().collect()
}

#[cfg(test)]
mod odd_even_tests {
    use std::collections::HashMap;

    use crate::integers::odd_even;
    #[test]
    fn test_odd_or_even() {
        // Given
        let cases = HashMap::from([
            (1, "Odd"),
            (2, "Even"),
            (3, "Odd"),
            (4, "Even"), // when no 'x' and 'o' is present should return true,
            (5, "Odd"),
        ]);

        for (input, want) in cases.into_iter() {
            // When
            let got = odd_even::even_or_odd(input);
            // Then
            assert_eq!(want, got, "value: {}", input);
        }
    }

    #[test]
    fn test_odd_or_even_smart() {
        // Given
        let cases = HashMap::from([
            (1, "Odd"),
            (2, "Even"),
            (3, "Odd"),
            (4, "Even"), // when no 'x' and 'o' is present should return true,
            (5, "Odd"),
        ]);

        for (input, want) in cases.into_iter() {
            // When
            let got = odd_even::even_or_odd_smart(input);
            // Then
            assert_eq!(want, got, "value: {}", input);
        }
    }

    #[test]
    fn test_sort_odds() {
        assert_eq!(odd_even::sort_odd_array(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
        assert_eq!(odd_even::sort_odd_array(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
        assert_eq!(odd_even::sort_odd_array(&[]), []);
        assert_eq!(odd_even::sort_odd_array(&[7,1]), [1,7]);
        assert_eq!(odd_even::sort_odd_array(&[5, 8, 6, 3, 4]), [3, 8, 6, 5, 4]);
        assert_eq!(odd_even::sort_odd_array(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), [1, 8, 3, 6, 5, 4, 7, 2, 9, 0]);
    }

    #[test]
    fn test_sort_odds_smart() {
        assert_eq!(odd_even::sort_odd_array_smart(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
        assert_eq!(odd_even::sort_odd_array_smart(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
        assert_eq!(odd_even::sort_odd_array_smart(&[]), []);
        assert_eq!(odd_even::sort_odd_array_smart(&[7,1]), [1,7]);
        assert_eq!(odd_even::sort_odd_array_smart(&[5, 8, 6, 3, 4]), [3, 8, 6, 5, 4]);
        assert_eq!(odd_even::sort_odd_array_smart(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]), [1, 8, 3, 6, 5, 4, 7, 2, 9, 0]);
    }
}
