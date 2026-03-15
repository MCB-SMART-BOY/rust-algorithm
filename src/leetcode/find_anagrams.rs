pub struct Solution;
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let s_bytes = s.as_bytes();
        let p_bytes = p.as_bytes();
        let s_len = s.len();
        let p_len = p.len();
        let mut result: Vec<i32> = Vec::new();

        if s_len < p_len {
            return result;
        }

        let mut p_freq = [0; 26];
        let mut window = [0; 26];

        for i in 0..p_len {
            let p_char_idx = (p_bytes[i] - b'a') as usize;
            let s_char_idx = (s_bytes[i] - b'a') as usize;
            p_freq[p_char_idx] += 1;
            window[s_char_idx] += 1;
        }

        if window == p_freq {
            result.push(0);
        }

        for start in 1..=s_len - p_len {
            let left_char = s_bytes[start - 1];
            let left_char_idx = (left_char - b'a') as usize;
            window[left_char_idx] -= 1;

            let end = start + p_len - 1;
            let right_char = s_bytes[end];
            let right_char_idx = (right_char - b'a') as usize;
            window[right_char_idx] += 1;

            if window == p_freq {
                result.push(start as i32);
            }
        }

        result
    }
}

pub fn test() {
    let input_s = "cbaebabacd".to_string();
    let input_p = "abc".to_string();
    let result = Solution::find_anagrams(input_s, input_p);
    println!("题目结果：{result:?}");
}
