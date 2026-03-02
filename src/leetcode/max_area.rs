pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut left = 0;
        let mut right = height.len() - 1;

        while left != right {
            max = max.max(height[left].min(height[right]) * (right - left) as i32);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        max
    }
}

pub fn test() {
    let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let result = Solution::max_area(input);
    println!("题目结果：{result:?}");
}
