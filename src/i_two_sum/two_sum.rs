use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::with_capacity(2);
    for (first_index, first_value) in nums.iter().enumerate() {
        for (second_index, second_value) in nums.iter().enumerate() {
            if second_index <= first_index  {
                continue;
            }

            if (first_value + second_value) == target {
                result.insert(0, first_index as i32);
                result.insert(1, second_index as i32);

                return result;
            }
        }
    }

    result
}

fn two_sum_smart(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut results: HashMap<i32, i32> = HashMap::new();
    for i in 0..nums.len() {
        match results.get(&nums[i]) {
            Some(&x) => return vec![x, i as i32],
            None => results.insert(target - nums[i], i as i32),
        };
    }

    return vec![-1,-1];
}

#[cfg(test)]
mod two_sum_tests {
    use crate::i_two_sum::two_sum;
    
    #[test]
    fn test_two_sum() {
        // Given
        let target = 9;
        let nums = vec![2,7,11,15];
        let want = vec![0, 1];
        // When
        let got = two_sum::two_sum(nums, target);
        // Then
        assert_eq!(want, got);
    }
    
    #[test]
    fn test_two_sum_smart() {
        // Given
        let target = 9;
        let nums = vec![2,7,11,15];
        let want = vec![0, 1];
        // When
        let got = two_sum::two_sum_smart(nums, target);
        // Then
        assert_eq!(want, got);
    }
    
    #[test]
    fn test_two_sum_two() {
        // Given
        let target = 6;
        let nums = vec![3, 2, 4];
        let want = vec![1, 2];
        // When
        let got = two_sum::two_sum(nums, target);
        // Then
        assert_eq!(want, got);
    }
    
    #[test]
    fn test_two_sum_two_smart() {
        // Given
        let target = 6;
        let nums = vec![3, 2, 4];
        let want = vec![1, 2];
        // When
        let got = two_sum::two_sum_smart(nums, target);
        // Then
        assert_eq!(want, got);
    }
}