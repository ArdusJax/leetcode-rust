pub struct Solution {}
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 1 as usize;
        for j in 0..nums.len() {
            if nums[i - 1] != nums[j] {
                nums[i] = nums[j];
                i += 1;
            }
        }
        i as i32
    }

    pub fn remove_duplicates_with_while(nums: &mut Vec<i32>) -> i32 {
        let (mut i, mut j) = (1 as usize, 0 as usize);
        while j < nums.len() {
            if nums[i - 1] != nums[j] {
                nums[i] = nums[j];
                i += 1;
            }
            j += 1;
        }
        i as i32
    }

    pub fn remove_duplicates_with_match(nums: &mut Vec<i32>) -> i32 {
        match nums.is_empty() {
            true => 0,
            false => {
                let mut prev = 0;
                for i in 1..nums.len() {
                    if nums[prev] != nums[i] {
                        prev += 1;
                        nums[prev] = nums[i];
                    }
                }
                (prev + 1) as i32
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&mut vec![1,1,2] => 2; "example 1 from leetcode question")]
    #[test_case(&mut vec![0,0,1,1,1,2,2,3,3,4] => 5; "example 2 from leetcode question")]
    fn removes_duplicates(nums: &mut Vec<i32>) -> i32 {
        Solution::remove_duplicates(nums)
    }
    #[test_case(&mut vec![1,1,2] => 2; "example 1 from leetcode question")]
    #[test_case(&mut vec![0,0,1,1,1,2,2,3,3,4] => 5; "example 2 from leetcode question")]
    fn removes_duplicates_double(nums: &mut Vec<i32>) -> i32 {
        Solution::remove_duplicates_with_while(nums)
    }
    #[test_case(&mut vec![1,1,2] => 2; "example 1 from leetcode question")]
    #[test_case(&mut vec![0,0,1,1,1,2,2,3,3,4] => 5; "example 2 from leetcode question")]
    fn removes_duplicates_with_match(nums: &mut Vec<i32>) -> i32 {
        Solution::remove_duplicates_with_match(nums)
    }
}
