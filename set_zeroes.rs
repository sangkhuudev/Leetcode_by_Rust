struct Solution;
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>)  {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut row_to_zeroes = vec![false;m];
        let mut col_to_zeroes = vec![false;n];

        // find the row and column to be zero
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    row_to_zeroes[i] = true;
                    col_to_zeroes[j] = true;
                }
            }
        }
        // set row ith to be zero
        for i in 0..m {
            if row_to_zeroes[i] {
                for j in 0..n {
                    matrix[i][j] = 0;
                }
            }
        }
        // set column jth to be zero
        for j in 0..n {
            if col_to_zeroes[j] {
                for i in 0..m {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}

fn main() {
    let mut matrix = vec![vec![0,1,2,0],vec![3,4,5,2],vec![1,3,1,5]];
    Solution::set_zeroes(&mut matrix);
    println!("{:?}",matrix)
}