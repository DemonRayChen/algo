pub struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut cursor = 1;
        for &d in digits.iter().rev() {
            let t = d + cursor;
            cursor = t / 10;
            ans.push(t % 10);
        }
        if cursor > 0 {
            ans.push(cursor);
        }
        ans.reverse();
        ans
    }

    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }
        let mut max_idx = 0;
        for i in 1..arr.len() {
            if arr[i] > arr[max_idx] {
                max_idx = i;
            }
        }
        if max_idx == 0 || max_idx == arr.len() - 1 {
            return false;
        }
        for i in (0..max_idx).rev() {
            if arr[i] >= arr[i + 1] {
                return false;
            }
        }
        for i in (max_idx + 1)..arr.len() {
            if (arr[i] >= arr[i - 1]) {
                return false;
            }
        }
        true
    }
}
