fn inverse_slice<T: Clone>(input: &[T], a: usize, b: usize) -> Vec<T> {
    if input.is_empty() {
        return vec![];
    }

    if  a > input.len() {
        return input.to_vec();
    }

    if a == 0 {
        return input[b..].to_vec();
    }

    if b > input.len() {
        return input[..a].to_vec();
    }

    let left = &input[0..a];
    let right = &input[b..];
    [left, right].concat()
}

fn inverse_slice_smart<T: Clone>(input: &[T], a: usize, b: usize) -> Vec<T> {
    input
        .iter()
        .take(a)
        .chain(input.iter().skip(b))
        .cloned()
        .collect()
}

fn inverse_slice_two<T: Clone>(input: &[T], a: usize, b: usize) -> Vec<T> {
    [input.get(..a).unwrap_or(input), input.get(b..).unwrap_or(&[])].concat().to_vec()
}

#[cfg(test)]
mod integer_slice_tests {
    use crate::slices::integers::inverse_slice;
    use crate::slices::integers::inverse_slice_smart;

    #[test]
    fn test_inverse_slice() {
        assert_eq!(inverse_slice(&[12, 14, 63, 72, 55, 24], 2, 4), [12, 14, 55, 24]);
        assert_eq!(inverse_slice(&[12, 14, 63, 72, 55, 24], 0, 3), [72, 55, 24]);
        assert_eq!(
            inverse_slice(&["Intuition", "is", "a", "poor", "guide", "when", "facing", "probabilistic", "evidence"], 5, 13),
            ["Intuition", "is", "a", "poor", "guide"]);
        assert_eq!(inverse_slice::<i32>(&[], 1, 4), []);
        assert_eq!(inverse_slice(&[0, 0, 0, 1, 0], 0, 3), [1, 0]);
    }

    #[test]
    fn test_inverse_slice_smart() {
        assert_eq!(inverse_slice_smart(&[12, 14, 63, 72, 55, 24], 2, 4), [12, 14, 55, 24]);
        assert_eq!(inverse_slice_smart(&[12, 14, 63, 72, 55, 24], 0, 3), [72, 55, 24]);
        assert_eq!(
            inverse_slice_smart(&["Intuition", "is", "a", "poor", "guide", "when", "facing", "probabilistic", "evidence"], 5, 13),
            ["Intuition", "is", "a", "poor", "guide"]);
        assert_eq!(inverse_slice_smart::<i32>(&[], 1, 4), []);
        assert_eq!(inverse_slice_smart(&[0, 0, 0, 1, 0], 0, 3), [1, 0]);
    }
}