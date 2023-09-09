struct Solution {}

impl Solution {
    // This times out in LeetCode and has a runtime of O(n^2)
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut profit, mut i) = (0, 0);

        while i < prices.len() {
            let mut idx = i + 1;
            while idx < prices.len() {
                let curr = prices[idx] - prices[i];
                if curr > profit {
                    profit = curr;
                }
                idx += 1;
            }
            i += 1;
        }
        profit
    }
    // This is a better solution and has an O(n) runtime
    // Runtime: 13ms
    // Memory: 3.1MB
    pub fn max_profit_fast(prices: Vec<i32>) -> i32 {
        let (mut profit, mut i) = (0, prices[0]);
        for j in &prices[1..] {
            if *j > i {
                profit = std::cmp::max(profit, j - i);
            } else {
                i = *j;
            }
        }
        profit
    }
    // Another solution using my favorite iter/fold
    // Runtime: 4ms
    // Memory: 2.9MB
    pub fn max_profit_faster(prices: Vec<i32>) -> i32 {
        let mut i = prices[0];
        prices.into_iter().fold(0, |mut profit, j| {
            if j > i {
                profit = std::cmp::max(profit, j - i);
            } else {
                i = j;
            };
            profit
        })
    }
    /// Best Time to Buy and Sell - Part II
    /// Make the most profit using multiple transactions
    /// Constraints: You can only hold one stock at a time
    /// Possible solution: Use a Vec instead of an i32 to track the transactions
    pub fn max_profits_mult_transactions(prices: Vec<i32>) -> i32 {
        let mut i = prices[0];
        let profits = prices.into_iter().fold(Vec::new(), |mut profits, j| {
            if j > i {
                profits.push(j - i);
                i = j;
            } else {
                i = j;
            };
            profits
        });
        profits.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_profit_succeeds1() {
        let input = vec![7, 1, 5, 3, 6, 4];
        let res = Solution::max_profit(input);
        assert_eq!(5, res);
    }
    #[test]
    fn max_profit_succeeds2() {
        let input = vec![7, 6, 4, 3, 1];
        let res = Solution::max_profit(input);
        assert_eq!(0, res);
    }
    #[test]
    fn max_profit_fast_succeeds1() {
        let input = vec![7, 6, 4, 3, 1];
        let res = Solution::max_profit_fast(input);
        assert_eq!(0, res);
    }
    #[test]
    fn max_profit_fast_succeeds2() {
        let input = vec![7, 1, 5, 3, 6, 4];
        let res = Solution::max_profit(input);
        assert_eq!(5, res);
    }
    #[test]
    fn max_profit_faster_succeeds1() {
        let input = vec![7, 6, 4, 3, 1];
        let res = Solution::max_profit_faster(input);
        assert_eq!(0, res);
    }
    #[test]
    fn max_profit_faster_succeeds2() {
        let input = vec![7, 1, 5, 3, 6, 4];
        let res = Solution::max_profit_faster(input);
        assert_eq!(5, res);
    }
    #[test]
    fn max_profits_mult_transactions_succeeds1() {
        let input = vec![7, 1, 5, 3, 6, 4];
        let res = Solution::max_profits_mult_transactions(input);
        assert_eq!(7, res);
    }
    #[test]
    fn max_profits_mult_transactions_succeeds2() {
        let input = vec![1, 2, 3, 4, 5];
        let res = Solution::max_profits_mult_transactions(input);
        assert_eq!(4, res);
    }
    #[test]
    fn max_profits_mult_transactions_succeeds3() {
        let input = vec![7, 6, 4, 3, 1];
        let res = Solution::max_profits_mult_transactions(input);
        assert_eq!(0, res);
    }
}
