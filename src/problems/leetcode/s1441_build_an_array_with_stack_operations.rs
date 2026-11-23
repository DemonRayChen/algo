//! # 1441. Build an Array with Stack Operations
//! 来源: LeetCode | 链接: https://leetcode.com/problems/build-an-array-with-stack-operations/
//! 难度: Medium ｜ 标签: Array Stack
//! 归属: Quest · Stack - Q1 | Date: 2026-11-23

pub struct Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        // A: 时间 O(n k) ~ O(n^2) 空间O(1)
        //let len = target.len();
        //let end_num = target[len - 1];
        //(1..=end_num)
        //    .into_iter()
        //    .flat_map(|x| {
        //        if target.contains(&x) {
        //            vec!["Push".to_string()]
        //        } else {
        //            vec!["Push".to_string(), "Pop".to_string()]
        //        }
        //    })
        //    .collect()

        // B: 相邻元素差值 时间 O(n) 空间 O(n)
        let mut res = Vec::new();
        let mut prev = 0;
        for &val in &target {
            for _ in 0..(val - prev - 1) {
                res.push("Push".to_string());
                res.push("Pop".to_string());
            }
            res.push("Push".to_string());
            prev = val;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::build_array(vec![1, 3], 3),
            vec![
                "Push".to_string(),
                "Push".to_string(),
                "Pop".to_string(),
                "Push".to_string()
            ]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::build_array(vec![1, 3], 3),
            vec![
                "Push".to_string(),
                "Push".to_string(),
                "Pop".to_string(),
                "Push".to_string()
            ]
        );
    }
}
