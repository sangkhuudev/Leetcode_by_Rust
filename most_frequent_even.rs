use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut res = -1;
        let mut max_frequency = 0;
        for num in nums {
            if num%2 == 0 {
                *map.entry(num).or_insert(0) +=1;
                if *map.entry(num).or_default() > max_frequency {
                    max_frequency = *map.entry(num).or_default();
                    res = num;
                } else if max_frequency == *map.entry(num).or_default() && res > num {
                    res = num;
                }
            }
        }
        res
    }    
}

fn main() {
    let res = Solution::most_frequent_even(vec![3,3,2,5,2,3,3,8,8,8]);  
    println!("{:?}", res);
}  