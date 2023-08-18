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
        let res = word1.chars().into_iter().fold(String::new(), |mut acc, c| {
            acc = format!("{}{}{}", acc, c, word2.get(idx..=idx).unwrap());
            idx += 1;
            acc
        });
        let rest = word2.split_at(idx);
        format!("{}{}", res, rest.1)
    }

    pub fn merge_alternately_append_overflow(word1: String, word2: String) -> String {
        let mut res = String::new();
        let (mut iter1, mut iter2) = (word1.chars(), word2.chars());
        while let Some(c) = iter1.next() {
            if let Some(cc) = iter2.next() {
                res = format!("{}{}", c, cc);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_alt_simple() {
        let s1 = String::from("ab");
        let s2 = String::from("pqrs");

        let res = Solution::merge_alternately_without_zip(s1, s2);
        assert_eq!(res, "apbqrs");
    }
    #[test]
    fn merge_alt_simple_second_string_bigger() {
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
