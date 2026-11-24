//! # 1475. Final Prices With a Special Discount in a Shop
//! 来源: LeetCode | 链接: https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop
//! 难度: Easy ｜ 标签: Array Stack
//! 归属: Quest · Monotonic Stack - Q1 | Date: 2026-11-24

pub struct Solution;

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        // 利用 stack 的单调性来进行处理, stack 保存下标,更好实现; 时间 O(n) 空间 O(n)
        let mut statck = Vec::new();
        let n = prices.len();
        let mut ans = prices.clone();
        for i in 0..n {
            while let Some(&top) = statck.last() {
                if prices[i] <= prices[top] {
                    let popped = statck.pop().unwrap();
                    ans[popped] = prices[popped] - prices[i];
                } else {
                    break;
                }
            }
            statck.push(i);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::final_prices(vec![8, 4, 6, 2, 3]),
            vec![4, 2, 4, 2, 3]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::final_prices(vec![1, 2, 3, 4, 5]),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::final_prices(vec![10, 1, 1, 6]), vec![9, 0, 1, 6]);
    }
}
