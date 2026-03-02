use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let num_set: HashSet<i32> = nums.iter().cloned().collect();
        let mut longest = 0;
        for &num in &num_set {
            if !num_set.contains(&(num - 1)) {
                let mut current_num = num;
                let mut current_length = 1;

                while num_set.contains(&(current_num + 1)) {
                    current_num += 1;
                    current_length += 1;
                }
                longest = longest.max(current_length);
            }
        }
        longest
    }
}

pub fn test() {
    let input = vec![100, 4, 200, 1, 3, 2];
    let result = Solution::longest_consecutive(input);
    println!("题目结果：{result:?}");
}
