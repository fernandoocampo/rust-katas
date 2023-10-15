use std::collections::HashMap;

/// Given an array of integers, find the one that appears an odd number of times.
/// There will always be only one integer that appears an odd number of times.
fn find_odd(input: &[i32]) -> i32 {
    if input.len() == 1 {
        return input[0];
    }
    let mut index = HashMap::<i32, usize>::new();
    for value in input.into_iter() {
        *index.entry(*value).or_insert(0) += 1;
    }

    let mut result: i32 = 0;

    for (k, v) in index {
        if (v % 2) == 0 {
            continue;
        }
        result = k;
        break;
    }

    result
}

fn find_odd_codewars(input: &[i32]) -> i32 {
    let result = input.iter().fold(0_i32, |a,v| {
        println!("a: {}, v: {}, a^v: {}", a, v, a^v);
        a^v
    });

    result
}

#[cfg(test)]
mod vec_tests {
    use std::collections::HashMap;
    use crate::colls::vecs;

    #[test]
    fn test_find_odd_int() {
        // Given
        let cases = HashMap::from([
            ("7=[7]", (7,vec![7])),
            ("0=[0]", (0,vec![0])),
            ("2=[1,1,2]", (2,vec![1,1,2])),
            ("0=[0,1,0,1,0]", (0,vec![0,1,0,1,0])),
            ("4=[1,2,2,3,3,3,4,3,3,3,2,2,1]", (4,vec![1,2,2,3,3,3,4,3,3,3,2,2,1])),
        ]);

        for (a_case, given_data) in cases {
            // When
            let got = vecs::find_odd(&given_data.1);
            // Then
            assert_eq!(given_data.0, got, "expected: {}", a_case);
        }
    }

    #[test]
    fn test_find_odd_int_codewars() {
        // Given
        let cases = HashMap::from([
            ("7=[7]", (7,vec![7])),
            ("0=[0]", (0,vec![0])),
            ("2=[1,1,2]", (2,vec![1,1,2])),
            ("0=[0,1,0,1,0]", (0,vec![0,1,0,1,0])),
            ("4=[1,2,2,3,3,3,4,3,3,3,2,2,1]", (4,vec![1,2,2,3,3,3,4,3,3,3,2,2,1])),
        ]);

        for (a_case, given_data) in cases {
            // When
            let got = vecs::find_odd_codewars(&given_data.1);
            // Then
            assert_eq!(given_data.0, got, "expected: {}", a_case);
        }
    }

    // #[test]
    // fn test_
}