struct Solution;
fn guess(num: i32) -> i32 {
    6
}
impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        let mut mid: i32;
        while left < right {
            mid = left + (right - left) / 2;
            let result = guess(mid);
            if result == 0 {
                return mid;
            } else if result == -1 {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        mid = left;
        mid
    }
}

fn main() {
    let res = unsafe { Solution::guessNumber(10) };
    println!("{}", res);
}
