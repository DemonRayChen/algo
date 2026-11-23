//! # 0150. Evaluate Reverse Polish Notation
//! 来源: LeetCode | 链接: https://leetcode.com/problems/evaluate-reverse-polish-notation
//! 难度: Medium ｜ 标签: Array Stack
//! 归属: Quest · Stack - Q2 | Date: 2026-11-23

pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        // A: 模拟 Stack 时间O(n) 空间 O(n)
        // let mut procs = Vec::new();
        // let mut last_idx = 0;
        // for x in &tokens {
        //     match x.parse::<i32>() {
        //         Ok(n) => {
        //             if last_idx == procs.len() {
        //                 procs.push(n);
        //             } else {
        //                 procs[last_idx] = n;
        //             }
        //             last_idx += 1;
        //         }
        //         Err(_) => {
        //             let prev = procs[last_idx - 2];
        //             let next = procs[last_idx - 1];
        //             let res = match x.as_str() {
        //                 "+" => prev + next,
        //                 "-" => prev - next,
        //                 "/" => prev / next,
        //                 "*" => prev * next,
        //                 _ => 0,
        //             };
        //             procs[last_idx - 2] = res;
        //             last_idx -= 1;
        //         }
        //     }
        // }
        // procs[0]

        // B: 使用真实的Stack 时间 O(N)  空间O(N)
        // let mut stack: Vec<i32> = Vec::new();
        // for token in &tokens {
        //     match token.as_str() {
        //         "+" | "-" | "*" | "/" => {
        //             let b = stack.pop().unwrap();
        //             let a = stack.pop().unwrap();
        //             let res = match token.as_str() {
        //                 "+" => a + b,
        //                 "-" => a - b,
        //                 "*" => a * b,
        //                 "/" => a / b,
        //                 _ => unreachable!(),
        //             };
        //             stack.push(res);
        //         }
        //         _ => {
        //             stack.push(token.parse().unwrap());
        //         }
        //     }
        // }
        // stack[0]

        // C: 使用 fold 替换 for 时间 O(N)  空间O(N)
        *tokens
            .iter()
            .fold(Vec::new(), |mut stack, token| {
                match token.as_str() {
                    "+" | "-" | "*" | "/" => {
                        let b = stack.pop().unwrap();
                        let a = stack.pop().unwrap();
                        let res = match token.as_str() {
                            "+" => a + b,
                            "-" => a - b,
                            "*" => a * b,
                            "/" => a / b,
                            _ => unreachable!(),
                        };
                        stack.push(res);
                    }
                    _ => {
                        stack.push(token.parse().unwrap());
                    }
                }
                stack
            })
            .first()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::eval_rpn(vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "*".to_string()
            ]),
            9
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::eval_rpn(vec![
                "4".to_string(),
                "13".to_string(),
                "5".to_string(),
                "/".to_string(),
                "+".to_string()
            ]),
            6
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::eval_rpn(vec![
                "10".to_string(),
                "6".to_string(),
                "9".to_string(),
                "3".to_string(),
                "+".to_string(),
                "-11".to_string(),
                "*".to_string(),
                "/".to_string(),
                "*".to_string(),
                "17".to_string(),
                "+".to_string(),
                "5".to_string(),
                "+".to_string()
            ]),
            22
        );
    }
}
