use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut res = vec![];
        let threshold = nums.len()/3;
        for i in 0..nums.len() {
            *map.entry(nums[i]).or_insert(0) +=1;
        }
        for (key,value) in map {
            if value > threshold {
                res.push(key);
            }
        }
        res
    }
}
fn main() {
    let res = Solution::majority_element(vec![3,3,3,3,3,4,4,4,8,8,8]); 
    println!("{:?}", res)
}