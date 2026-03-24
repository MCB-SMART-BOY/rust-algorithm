pub struct Solution;
impl Solution {
    pub fn rotate(nums: &mut [i32], k: i32) {
        let n = nums.len();
        if n == 0 {
            return;
        }
        nums.rotate_right((k as usize) % n);
    }
}
// impl Solution {
//     pub fn rotate(nums: &mut Vec<i32>, k: i32) {
//         let n = nums.len();
//         let k = (k as usize) % n;

//         if k == 0 {
//             return;
//         }

//         nums.reverse();
//         nums[0..k].reverse();
//         nums[k..].reverse();
//     }
// }

pub fn test() {
    let mut input = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;
    Solution::rotate(&mut input, k);
    println!("{:?}", input);
}
