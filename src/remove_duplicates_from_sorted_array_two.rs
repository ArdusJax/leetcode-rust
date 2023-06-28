pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        const CONSECUTIVE_MAX: i32 = 2;
        let (mut i, mut j, mut count) = (0 as usize, 1 as usize, 1);
        while j < nums.len() {
            let identical = nums[i] == nums[j];
            if identical & (count < CONSECUTIVE_MAX) | !identical {
                nums[i + 1] = nums[j];
                i += 1;
                if identical {
                    count += 1;
                } else {
                    count = 1;
                }
            }
            j += 1;
        }
        i += 1;
        // Added this for a little insight into what the final vecl looks like
        dbg!(nums);
        i as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&mut vec![1,1,1,2,2,3] => 5;"leetcode example 1")]
    #[test_case(&mut vec![0,0,1,1,1,1,2,3,3] => 7;"leetcode example 2")]
    fn remove_with_max_of_two(nums: &mut Vec<i32>) -> i32 {
        Solution::remove_duplicates(nums)
    }
}
