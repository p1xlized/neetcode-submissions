use std::collections::HashMap;

impl Solution {
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut num_map = HashMap::new();
    for i in 0..nums.len() {
        num_map.entry(nums[i]).and_modify(|e| *e += 1).or_insert(1);
    }
    num_map
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| num)
        .unwrap_or(0)
}
}
