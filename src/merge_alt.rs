struct Solution;
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        word1
            .chars()
            .into_iter()
            .zip(word2.chars().into_iter())
            .fold(String::new(), |mut acc, c| {
                acc = format!("{}{}{}", acc, c.0, c.1);
                acc
            })
    }
    pub fn merge_alternately_without_zip(word1: String, word2: String) -> String {
        let mut idx: usize = 0;
        word1.chars().into_iter().fold(String::new(), |mut acc, c| {
            acc = format!("{}{}{}", acc, c, word2.get(idx..=idx).unwrap());
            idx += 1;
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_alt_simple() {
        let s1 = String::from("abc");
        let s2 = String::from("def");

        let res = Solution::merge_alternately_without_zip(s1, s2);
        assert_eq!(res, "adbecf");
    }
    #[test]
    fn merge_alt_simple_zip() {
        let s1 = String::from("abc");
        let s2 = String::from("def");

        let res = Solution::merge_alternately(s1, s2);
        assert_eq!(res, "adbecf");
    }
}
