impl Solution {
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    let mut seen = HashMap::new();


    for i in 0..nums.len() {
        let num = nums[i];
        let complement = target - num;

        if seen.contains_key(&complement) {

            let j = seen[&complement]; 
            return vec![j as i32, i as i32];
        }
        seen.insert(num, i);
    }

    vec![]
}
}
