
struct Solution;

impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();
        let mut rows = vec![vec![false; n]; n];
        let mut cols = vec![vec![false; n]; n];

        for i in 0..n {
            for j in 0..n {
                let num = matrix[i][j] as usize - 1;
                if num >= n || rows[i][num] || cols[j][num] {
                    return false;
                }
                rows[i][num] = true;
                cols[j][num] = true;
            }
        }

        true
    }
}

fn main() {
    let res = Solution::check_valid(vec![vec![3, 1, 2], vec![1, 2, 3], vec![3, 2, 1]]);
    println!("{}", res);
}
