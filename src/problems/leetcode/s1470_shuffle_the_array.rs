//! # 1470. Shuffle the Array
//! 来源: LeetCode | 链接: https://leetcode.com/problems/shuffle-the-array
//! 难度: Easy ｜ 标签: Array
//! 归属: Quest · Array I · Q2 | Date: 2026-11-20

pub struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        // A: 新增一个数组, 再依次push
        //let n = n as usize;
        //let mut ans = Vec::with_capacity(n * 2);
        //for i in 0..n {
        //    ans.push(nums[i]);
        //    ans.push(nums[i + n]);
        //}
        //ans

        // B: 使用iter 实现
        let n = n as usize;
        nums[..n] // 切片, 开闭区间; 闭区间使用 `..=`
            .iter() // 迭代器
            .zip(&nums[n..]) // 双列并排, 两列不同时 取短列的长度;
            .flat_map(|(&a, &b)| [a, b]) // 类似map，但是会多一个自动 iter 的功能
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        assert_eq!(
            Solution::shuffle(vec![1, 2, 3, 3, 2, 1], 3),
            vec![1, 3, 2, 2, 3, 1]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
            vec![1, 4, 2, 3, 3, 2, 4, 1]
        );
    }
}
