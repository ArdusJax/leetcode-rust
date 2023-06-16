pub struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) -> Vec<i32> {
        let mut m = m as usize;
        let mut n = n as usize;

        while n > 0 {
            if m > 0 && nums1[m - 1] > nums2[n - 1] {
                nums1[m + n - 1] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[m + n - 1] = nums2[n - 1];
                n -= 1;
            }
        }
        nums1.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_solution() {
        let mut a1 = vec![1, 2, 3, 0, 0, 0];
        let mut a2 = vec![2, 5, 6];
        assert_eq!(
            vec![1, 2, 2, 3, 5, 6],
            Solution::merge(&mut a1, 3, &mut a2, 3)
        );
    }
}
