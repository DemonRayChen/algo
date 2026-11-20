//! # 1929. Get Concatenation of Array
//! 来源: LeetCode | 链接: https://leetcode.com/problems/concatenation-of-array
//! 难度: Easy ｜ 标签: Array
//! 归属: Quest · Q1 | Date: 2026-11-20

pub struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        // A: 最原始的方案, 应该是所有语言通用
        //let n = nums.len();
        //let mut ans = vec![0; n * 2];
        //for i in 0..n * 2 {
        //    ans[i] = nums[i % n];
        //}
        //ans

        // B: 预分配capacity
        //let mut ans = Vec::with_capacity(nums.len() * 2);
        //for &x in &nums {
        //    ans.push(x);
        //}
        //for &x in &nums {
        //    ans.push(x);
        //}
        //ans

        // C: 使用iter 更正确的rust 的写法
        //nums.iter() // 转成迭代器流
        //    .chain(&nums) // 增加新的一个迭代器流接在上一个后面
        //    .copied() // 把每一项进行拆包 从 &T -> T
        //    .collect() // 消费整个流, 默认返回Vec<T>

        // D: rust 的标准库的repeat, 自我复制
        nums.repeat(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::get_concatenation(vec![1, 2, 3]),
            vec![1, 2, 3, 1, 2, 3]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::get_concatenation(vec![1, 3, 2, 1]),
            vec![1, 3, 2, 1, 1, 3, 2, 1]
        );
    }
}
