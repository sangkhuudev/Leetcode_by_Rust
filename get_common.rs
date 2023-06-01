use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let set: HashSet<_> = nums1.into_iter().collect();
        let mut res : Vec<i32> = vec![];
        for num in nums2 {
            if set.contains(&num)  {
                res.push(num)
            }
        }
        let min_common = res.iter().min();
        match min_common {
            Some(&min_common) => min_common,
            None => -1,
        }
    }
}
fn main() {
    let res = Solution::get_common(vec![9,3,4,8,6],vec![1,2,4,3,8,9]); // 3
    println!("{}", res)
}