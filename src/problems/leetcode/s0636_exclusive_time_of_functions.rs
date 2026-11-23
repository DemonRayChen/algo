//! # 0150. Exclusive Time of Functions
//! 来源: LeetCode | 链接: https://leetcode.com/problems/exclusive-time-of-functions
//! 难度: Medium ｜ 标签: Array Stack
//! 归属: Quest · Stack - Q3 | Date: 2026-11-23

pub struct Solution;

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        // A: 模拟栈
        // let mut stack: Vec<(i32, &str, i32)> = Vec::new();
        // let mut ans = vec![0; n as usize];
        // let mut last_ts = 0;
        // for log in logs.iter().map(|log| {
        //     let s_arr: Vec<&str> = log.split(":").collect();
        //     let pid: i32 = s_arr[0].parse().unwrap();
        //     let ts: i32 = s_arr[2].parse().unwrap();
        //     (pid, s_arr[1], ts)
        // }) {
        //     if log.1 == "start" {
        //         if !stack.is_empty() {
        //             let prev = stack.pop().unwrap();
        //             ans[prev.0 as usize] += log.2 - last_ts - 1;
        //             stack.push(prev);
        //         }
        //         stack.push(log);
        //         last_ts = log.2;
        //         ans[log.0 as usize] += 1;
        //     } else {
        //         let _ = stack.pop().unwrap();
        //         ans[log.0 as usize] += log.2 - last_ts;
        //         last_ts = log.2;
        //     }
        // }
        // ans

        // 优化代码
        let mut ans = vec![0; n as usize];
        let mut stack: Vec<i32> = Vec::new(); // 只存 pid
        let mut prev_ts = 0;

        for log in &logs {
            let mut parts = log.splitn(3, ':'); // 固定 3 段，不 collect
            let pid: i32 = parts.next().unwrap().parse().unwrap();
            let is_start = parts.next().unwrap() == "start";
            let ts: i32 = parts.next().unwrap().parse().unwrap();

            if is_start {
                // 上一个函数从 prev_ts 跑到 ts-1
                if let Some(&top) = stack.last() {
                    ans[top as usize] += ts - prev_ts;
                }
                stack.push(pid);
                prev_ts = ts;
            } else {
                // 当前函数从 prev_ts 跑到 ts（含）
                let top = stack.pop().unwrap();
                ans[top as usize] += ts - prev_ts + 1;
                prev_ts = ts + 1; // 下一个函数从 ts+1 开始
            }
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
            Solution::exclusive_time(
                2,
                vec![
                    "0:start:0".to_string(),
                    "1:start:2".to_string(),
                    "1:end:5".to_string(),
                    "0:end:6".to_string()
                ]
            ),
            vec![3, 4]
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::exclusive_time(
                1,
                vec![
                    "0:start:0".to_string(),
                    "0:start:2".to_string(),
                    "0:end:5".to_string(),
                    "0:start:6".to_string(),
                    "0:end:6".to_string(),
                    "0:end:7".to_string()
                ]
            ),
            vec![8]
        )
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::exclusive_time(
                2,
                vec![
                    "0:start:0".to_string(),
                    "0:start:2".to_string(),
                    "0:end:5".to_string(),
                    "1:start:6".to_string(),
                    "1:end:6".to_string(),
                    "0:end:7".to_string()
                ]
            ),
            vec![7, 1]
        )
    }
}
