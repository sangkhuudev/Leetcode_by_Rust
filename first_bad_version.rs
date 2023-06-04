struct Solution;
impl Solution {
    pub fn is_bad_version(&self , version : i32) -> bool {
        version >= 1
    }
    pub fn first_bad_version(&self , n : i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        while left < right {
            let mid = left + (right - left)/2 ;
            if self.is_bad_version(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}
fn main() {
    let solution = Solution;
    let total_version = 10;
    let first_bad = solution.first_bad_version(total_version);
    println!("{}", first_bad);

}