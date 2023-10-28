fn make_fibos(n: usize) -> Vec<i32> {
    let mut result = Vec::new();
    if n == 0 {
        return result;
    }

    let mut before_last = 0;

    if n == 1 {
        result.push(before_last);
        return result;
    }

    let mut last_one = 1;
    if n == 2 {
        result.push(last_one);
        return result;
    }

    result.push(before_last);
    result.push(last_one);
    for i in 2..n {
        result.push(before_last + last_one);
        before_last = last_one;
        last_one = result[i];
    }

    result
}

#[cfg(test)]
mod fibo_tests {
    use crate::integers;

    #[test]
    fn test_make_fibos() {
        // given
        let cases = std::collections::HashMap::from([
            ("011235", (6, vec![0, 1, 1, 2, 3, 5])),
            ("0112358", (7, vec![0, 1, 1, 2, 3, 5, 8])),
            ("011235813", (8, vec![0, 1, 1, 2, 3, 5, 8, 13])),
            ("01123581321", (9, vec![0, 1, 1, 2, 3, 5, 8, 13, 21])),
            ("0112358132134", (10, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34])),
        ]);

        for (test_name, test_data) in cases.into_iter() {
            // when
            let got = integers::fibbo::make_fibos(test_data.0);
            // then
            assert_eq!(got, test_data.1, "test: {}", test_name);
        }
    }
}
