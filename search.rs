struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = left + (right - left) / 2;

            if nums[mid] == target {
                return mid as i32;
            }

            if nums[left] < nums[mid] {
                if target >= nums[left] && target < nums[mid] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else if nums[left] > nums[mid] {
                if target > nums[mid] && target <= nums[right] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            } else {
                left += 1;
            }
        }

        -1
    }
}

fn main() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 0;
    let result = Solution::search(nums, target);
    println!("Target index: {}", result);
}
