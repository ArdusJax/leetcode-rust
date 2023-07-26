pub struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut majority, mut count) = (0, 0);

        nums.into_iter().for_each(|num| {
            if count == 0 {
                majority = num;
            }

            if majority == num {
                count += 1;
            } else {
                count -= 1;
            }
        });
        majority
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn small_array_test() {
        let small_test = vec![3, 2, 3];
        let result = Solution::majority_element(small_test);
        assert_eq!(result, 3);
    }
    #[test]
    pub fn large_array_test() {
        let large_test = vec![2, 2, 1, 1, 1, 2, 2];
        let result = Solution::majority_element(large_test);
        assert_eq!(result, 3);
    }
}
