
use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0, 0];
        let mut map = HashMap::new();

        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        for val in map.values() {
            res[0] += val/2;
            if val%2 != 0 {
                res[1] += 1;
            }   
        }
        res
    }
}

fn main() {
    let res = Solution::number_of_pairs(vec![3,3,2,5,2,2,3,3,8,8]);  //  [4,3]
    println!("{:?}", res);
}  