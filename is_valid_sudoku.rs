struct Solution;
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = vec![std::collections::HashSet::new(); 9];
        let mut cols = vec![std::collections::HashSet::new();9];
        let mut boxes = vec![std::collections::HashSet::new();9];
        for i in 0..9 {
            for j in 0..9 {
                let digit = board[i][j];
                if digit != '.' {
                    if !rows[i].insert(digit) || !cols[j].insert(digit) || !boxes[(i/3)*3 + j/3].insert(digit){
                        return false;
                    }  // this if clause checks row set , column set or box set contained the digit already!!!
                    // boxes[(i/3)*3 + j/3] identifies the cell (i,j) belongs to that box,note that i/3 is the quotient not decimal
                }
            }
        }
        true
    }
}

fn main() {
    let board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];

    let is_valid = Solution::is_valid_sudoku(board);
    println!("Is the Sudoku board valid? {}", is_valid);
}