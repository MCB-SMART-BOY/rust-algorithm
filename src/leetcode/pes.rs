pub struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![1; n];
        let (mut left_product, mut right_product) = (1, 1);
        for i in 0..n {
            let rev = n - i - 1;
            result[i] *= left_product;
            result[rev] *= right_product;
            left_product *= nums[i];
            right_product *= nums[rev];
        }

        result
    }
}

pub fn test() {
    let input = vec![1, 2, 3, 4];
    let result = Solution::product_except_self(input);
    println!("题目结果：{result:?}");
}
