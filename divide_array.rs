
use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn divide_array(nums : Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for num in nums.iter() {
            let count = map.entry(num).or_insert(0);
            *count+=1;
        }
        map.iter().all(|(_,val)|  val%2 == 0)
    }
}

fn main() {
    let res = Solution::divide_array(vec![3,3,2,5,5,2,3,3]);
    println!("{}", res)
}