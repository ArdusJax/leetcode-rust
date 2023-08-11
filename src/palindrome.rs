// Runtime: O(n) Space: O(1)
pub fn palendrome(text: &str) -> bool {
    let chars = text.chars().collect::<Vec<char>>();
    let mut head = 0;
    let mut tail: usize = chars.len() - 1;
    loop {
        if chars[head] != chars[tail] {
            return false;
        };
        head += 1;
        tail -= 1;

        if head == tail {
            return true;
        };
    }
}

pub fn palendrome_easymode(text: &str) -> bool {
    let reversed = text.chars().rev().collect::<String>();
    reversed == text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_a_palendrome() {
        let s = "bob";
        let t = "racecar";
        assert_eq!(palendrome(s), true);
        assert_eq!(palendrome(t), true);
    }
    #[test]
    fn is_not_a_palendrome() {
        let s = "bill";
        assert_eq!(palendrome(s), false);
    }
    #[test]
    fn is_an_easymode_palendrome() {
        let s = "bob";
        let t = "racecar";
        assert_eq!(palendrome_easymode(s), true);
        assert_eq!(palendrome_easymode(t), true);
    }
    #[test]
    fn is_not_an_easymode_palendrome() {
        let s = "bill";
        assert_eq!(palendrome_easymode(s), false);
    }
}
