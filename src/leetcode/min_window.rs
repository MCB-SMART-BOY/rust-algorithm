use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut target: HashMap<char, usize> = HashMap::with_capacity(52);
        let mut window: HashMap<char, usize> = HashMap::with_capacity(52);
        let (mut left, mut valid) = (0, 0);

        let (mut start, mut min_len) = (0, usize::MAX);
        let s_chars: Vec<char> = s.chars().collect();
        for c in t.chars() {
            *target.entry(c).or_insert(0) += 1;
        }
        let target_len = target.len();

        for (right, &c) in s_chars.iter().enumerate() {
            if target.contains_key(&c) {
                *window.entry(c).or_insert(0) += 1;
                if window[&c] == target[&c] {
                    valid += 1;
                }
            }
            while valid == target_len {
                let current_len = right - left + 1;
                if current_len < min_len {
                    start = left;
                    min_len = current_len;
                }
                let left_char = s_chars[left];
                if target.contains_key(&left_char) {
                    if window[&left_char] == target[&left_char] {
                        valid -= 1;
                    }
                    *window.get_mut(&left_char).unwrap() -= 1;
                }
                left += 1;
            }
        }
        if min_len == usize::MAX {
            String::new()
        } else {
            s_chars[start..(start + min_len)].iter().collect()
        }
    }
}

pub fn test() {
    let s = "ADOBECODEBANC".to_string();
    let t = "ABC".to_string();
    let result = Solution::min_window(s, t);
    println!("题目结果：{result:?}");
}
