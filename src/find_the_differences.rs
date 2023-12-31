pub struct Solution {}

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let init = s.as_bytes().iter().fold(0, |acc: u8, x: &u8| acc ^ (*x));
        let res = t.as_bytes().iter().fold(init, |acc: u8, x: &u8| acc ^ (*x));
        res as char
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ftd_example_1() {
        let s = String::from("abcd");
        let t = String::from("abecd");
        let v = s.as_bytes();
        let w = t.as_bytes();
        let v = v.iter().fold(0, |acc: u8, x: &u8| {
            println!("{acc} : {}", (*x));
            acc ^ (*x)
        });
        println!();
        let w = w.iter().fold(v, |acc: u8, x: &u8| {
            println!("{acc} : {}", (*x));
            acc ^ (*x)
        });
        dbg!(v, w);
        let res = Solution::find_the_difference(s, t);
        dbg!(res);
    }
    #[test]
    fn ftd_example_2() {
        let s = String::from("");
        let t = String::from("y");
        let res = Solution::find_the_difference(s, t);
        dbg!(res);
    }
    #[test]
    fn ftd_example_3() {
        let s = String::from("abcd");
        let t = String::from("abecd");
        let res = Solution::find_the_difference(s, t);
        dbg!(res);
    }
}
