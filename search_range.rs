struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let start_index = Solution::search_index(&nums,target,true);
        let end_index = Solution::search_index(&nums,target,false);
        if start_index > end_index {
            return vec![-1, -1];
        }
        vec![start_index, end_index]
    }
    fn search_index(nums: &[i32], target: i32, is_left: bool) -> i32 {
        let (mut start, mut end) = (0,nums.len() as i32 - 1);
        let mut res = -1;
        while start <= end {
            let mid = start + (end - start)/2 ;
            if nums[mid as usize] > target || (is_left && nums[mid as usize] == target) {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
            if nums[mid as usize] == target {
                res = mid;
            }
        }
        res
    }
}

fn main() {
    let nums = vec![1,2,2,3,4,4,4,5,6];
    let target = 4;
    let result = Solution::search_range(nums, target);
    println!("Target index: {:?}", result);
}
