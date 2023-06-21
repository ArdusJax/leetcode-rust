pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i: usize = 0;

        for j in 0..nums.len() {
            if nums[j] != val {
                nums[i] = nums[j];
                i += 1;
            }
        }

        dbg!(nums);
        i as i32
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn remove_element_succeeds() {
        let mut nums = vec![3, 2, 2, 3];
        let target = 3;

        let answer = Solution::remove_element(&mut nums, target);
        assert_eq!(2, answer);
    }

    #[test]
    fn remove_element_succeeds_again() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let target = 2;

        let answer = Solution::remove_element(&mut nums, target);
        assert_eq!(5, answer);
    }
    #[test]
    fn remove_element_from_empty() {
        let mut nums = vec![];
        let target = 2;

        let answer = Solution::remove_element(&mut nums, target);
        assert_eq!(0, answer);
    }
}
