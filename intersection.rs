use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn intersection(nums : Vec<Vec<i32>>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut res = vec![];
        for i in 0..nums.len() {
            for j in 0..nums[i].len() {
                *map.entry(nums[i][j]).or_insert(0) +=1;
            }
        }
        for (key,value) in &map {
            if *value == nums.len() {
                res.push(*key);
            }
        }
        res.sort();
        res
    }
}

fn main() {
    let res = Solution::intersection(vec![vec![3,1,2,4,5],vec![1,2,3,4],vec![3,4,5,6]]);
    println!("{:?}", res)
}