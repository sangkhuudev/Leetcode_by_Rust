struct Solution;
impl Solution {
    pub fn get_rows(row_index: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![1;row_index as usize + 1];
        for i in 1..row_index as usize + 1 {
            for j in (1..i).rev() {
                res[j] = res[j] + res[j-1]
            }
        }
        res
    }
}

fn main() {
    let res = Solution::get_rows(5);
    println!("{:?}", res);
}


 // row_index = 5
    // [1, 1, 1, 1, 1, 1]
    // [1, 2, 1, 1, 1, 1]
    // [1, 2, 3, 1, 1, 1]
    // [1, 3, 3, 1, 1, 1]
    // [1, 3, 3, 4, 1, 1]
    // [1, 3, 6, 4, 1, 1]
    // [1, 4, 6, 4, 1, 1]
    // [1, 4, 6, 4, 5, 1]
    // [1, 4, 6, 10, 5, 1]
    // [1, 5 , 10, 10, 5, 1]