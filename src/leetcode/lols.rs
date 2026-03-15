use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_index: HashMap<char, usize> = HashMap::new();
        let mut left = 0;
        let mut max_len = 0;
        let chars: Vec<char> = s.chars().collect();

        for (right, &c) in chars.iter().enumerate() {
            match char_index.insert(c, right) {
                Some(last_index) if last_index >= left => left = last_index + 1,
                _ => {}
            }
            max_len = max_len.max((right - left + 1) as i32);
        }
        max_len
    }
}
pub fn test() {
    let input = String::from("abcabcbb");
    let result = Solution::length_of_longest_substring(input);
    println!("题目结果：{result:?}");
}
