

struct Solution ;
impl Solution {
    pub fn find_subarrays(nums: Vec<i32>) -> bool {
        let mut seen = std::collections::HashSet::new();
        for i in 0..nums.len()-1 {
            let sum = nums[i]+nums[i+1];
            if seen.contains(&sum) {
                return true;
            }
            seen.insert(sum);
        }
    false  
    }  
}


fn main() {
    let res = Solution::find_subarrays(vec![1,-4,4,-7]);
    println!("{}", res)
}