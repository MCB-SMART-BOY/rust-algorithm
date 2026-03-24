pub struct Solution;
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut sorted_intervals = intervals;
        sorted_intervals.sort_unstable_by_key(|interval| interval[0]);

        let mut merge = Vec::new();
        merge.push(sorted_intervals[0].clone());

        for current in sorted_intervals.iter().skip(1) {
            let last = merge.last_mut().unwrap();
            let (current_start, current_end) = (current[0], current[1]);
            let last_end = last[1];

            if current_start <= last_end {
                last[1] = last_end.max(current_end);
            } else {
                merge.push(current.clone());
            }
        }

        merge
    }
}

pub fn test() {
    let input = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    let result = Solution::merge(input);
    println!("题目结果：{result:?}");
}
