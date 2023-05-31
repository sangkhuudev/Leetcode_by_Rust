use std::collections::HashMap;
use std::collections::HashSet;
struct Solution;
impl Solution {
    // Method 1 : use hashmap
    pub fn find_max_k(nums : Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut max =-1;
        for num in nums {
            *map.entry(num).or_insert(0)+=1;
        }
        for &key in map.keys() {
            if map.contains_key(&(-key)) && max < key {
                max = key;
            }
        }
        if max == -1 {
            return max;
        } else {
            return max.abs();
        }
        
    }
    // Method 2 : use hashset
    pub fn find_max_k2(nums : Vec<i32>) -> i32 {
        let mut max =-1;
        let mut set: HashSet<i32> = HashSet::new();
        for num in nums {
            if let Some(value) = set.get(&(-num)) {
                max = max.max(value.abs());
                continue;
            }
            set.insert(num);
        }
        max
    }
}

fn main() {
    let res = Solution::find_max_k2(vec![-37,37,-9,2,47,18,13,-11,9,-28,-47]);
    println!("{}", res)
}