impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l: isize = 0;
        let mut u: isize = nums.len() as isize - 1;

        while l <= u {
            let m = l + (u - l) / 2;
            
         
            if nums[m as usize] < target {
                l = m + 1;
            } else if nums[m as usize] == target {
                return m as i32; 
            } else {
                u = m - 1;
            }
        }
        
        -1
    }
}