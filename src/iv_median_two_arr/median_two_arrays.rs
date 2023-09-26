fn median_two_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    if nums1.is_empty() && nums2.is_empty() {
        return 0.0;
    }
    
    calculate_median(merge_and_order(nums1, nums2))
}

fn merge_and_order(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
    if nums1.is_empty() && !nums2.is_empty() {
        nums2.sort();

        return nums2;
    }

    if nums2.is_empty() && !nums1.is_empty() {
        nums1.sort();

        return nums1;
    }

    let mut major = nums1.clone();
    let mut minor = nums2.clone();

    if nums2[0] >= nums1[nums1.len()-1] {
        major = nums2;
        minor = nums1;
    }

    minor.append(&mut major);

    minor.sort();

    minor
}

fn calculate_median(nums: Vec<i32>) -> f64 {
    if nums.len()%2 != 0 {
        let idx = (nums.len() + 1) / 2;

        return nums[idx-1] as f64;
    }
    // even number
    let idx1 = (nums.len() / 2) - 1;
    let idx2 = idx1 + 1;
    let result = ((nums[idx1]+nums[idx2]) as f64) / 2.0;
    
    result
}

fn find_median_sorted_arrays_ii(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut merged: Vec<i32> = Vec::new();
    let (mut i, mut j) = (0, 0);
    
    while i < nums1.len() && j < nums2.len() {
        if nums1[i] < nums2[j] {
            merged.push(nums1[i]);
            i += 1;
        } else {
            merged.push(nums2[j]);
            j += 1;
        }
    }
    
    while i < nums1.len() {
        merged.push(nums1[i]);
        i += 1;
    }
    while j < nums2.len() {
        merged.push(nums2[j]);
        j += 1;
    }
    
    let mid = merged.len() / 2;
    if merged.len() % 2 == 0 {
        return (merged[mid-1] + merged[mid]) as f64 / 2.0;
    } else {
        return merged[mid] as f64;
    }
}

#[cfg(test)]
pub mod median_two_arrays_tests {
    use crate::iv_median_two_arr::median_two_arrays;
    #[test]
    fn test_median_two_arrays() {
        // Given
        let test_cases = std::collections::HashMap::from(
            [
                ("2.00000", (vec![1,3], vec![2], 2.00000)),
                ("2.00000", (vec![1,2], vec![3,4], 2.50000)),
            ]
        );

        for (test_case, arrs) in test_cases.into_iter() {
            // When
            let got = median_two_arrays::median_two_arrays(arrs.0, arrs.1);
            // Then
            assert_eq!(got, arrs.2, "{} want: {:?}, but got: {}", test_case, arrs.2, got)
        }
    }
}