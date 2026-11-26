//! # 1046. Last Stone Weight
//! 来源: LeetCode | 链接: https://leetcode.com/problems/last-stone-weight/description/
//! 难度: Easy ｜ 标签: Array Heap(Priority Queue)
//! 归属: Quest · Heap - Q1 | Date: 2026-11-26

use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        for &x in &stones {
            heap.push(x);
        }

        loop {
            if heap.len() <= 1 {
                break;
            }
            let res = heap.pop().unwrap() - heap.pop().unwrap();
            if res > 0 {
                heap.push(res);
            }
        }

        heap.pop().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::last_stone_weight(vec![1]), 1);
    }
}
