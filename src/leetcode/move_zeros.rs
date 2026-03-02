pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut slow = 0;
        for fast in 0..nums.len() {
            if nums[fast] != 0 {
                nums.swap(slow, fast);
                slow += 1;
            }
        }
    }
}

pub fn test() {
    let mut input = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut input);
    let result = input;
    println!("题目结果：{result:?}");
}
