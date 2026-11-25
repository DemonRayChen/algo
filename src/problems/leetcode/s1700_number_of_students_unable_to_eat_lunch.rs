//! # 1700. Number of Students Unable to Eat Lunch
//! 来源: LeetCode | 链接: https://leetcode.com/problems/number-of-students-unable-to-eat-lunch
//! 难度: Easy ｜ 标签: Array Stack Queue Simulation
//! 归属: Quest · Queue - Q1 | Date: 2026-11-25

use std::{collections::VecDeque, ffi::FromBytesUntilNulError, fmt::FromFn};

pub struct Solution;

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        // 模拟解法
        // let mut ans = students.len() as i32;
        // let mut students = students;
        // let n = students.len();
        // let mut queue_start_idx = 0;
        // for &sandwich in &sandwiches {
        //     let mut queue_query_cnt = 0;
        //     while queue_query_cnt < n {
        //         queue_query_cnt += 1;
        //
        //         if students[queue_start_idx] == -1 || students[queue_start_idx] != sandwich {
        //             queue_start_idx = (1 + queue_start_idx) % n;
        //             continue;
        //         }
        //
        //         students[queue_start_idx] = -1;
        //         ans -= 1;
        //         queue_start_idx = (1 + queue_start_idx) % n;
        //         break;
        //     }
        //     if queue_query_cnt >= n {
        //         return ans;
        //     }
        // }
        // ans

        // 更 rust 的模拟
        // let mut queue: VecDeque<i32> = students.into_iter().collect();
        // let mut stack: Vec<i32> = sandwiches.into_iter().rev().collect();
        // while let Some(&top) = stack.last() {
        //     if let Some(&front) = queue.front() {
        //         if top == front {
        //             queue.pop_front();
        //             stack.pop();
        //             continue;
        //         }
        //     }
        //     let want = queue.iter().any(|&s| s == top);
        //     if !want {
        //         break;
        //     }
        //
        //     if let Some(front) = queue.pop_front() {
        //         queue.push_back(front);
        //     }
        // }
        //
        // queue.len() as i32

        // 计算版
        let mut count = vec![0; 2];
        for &s in &students {
            count[s as usize] += 1;
        }

        for &s in &sandwiches {
            if count[s as usize] == 0 {
                return count[0] + count[1];
            }
            count[s as usize] -= 1;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 0, 0], vec![0, 0, 1, 1]),
            0
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
            3
        )
    }
}
