pub struct Solution;

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut stack = Vec::new();
        let s_chars: Vec<char> = s.chars().collect();
        stack.push(s_chars[0]);
        for i in 1..s_chars.len() {
            if stack.contains(&s_chars[i]) {
                continue;
            }
            while let Some(&top) = stack.last() {
                if top > s_chars[i] {
                    let mut exist_tail = false;
                    for j in i + 1..s_chars.len() {
                        if top == s_chars[j] {
                            exist_tail = true;
                            break;
                        }
                    }
                    if exist_tail {
                        stack.pop();
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            stack.push(s_chars[i]);
        }

        stack.into_iter().collect()
    }
}
