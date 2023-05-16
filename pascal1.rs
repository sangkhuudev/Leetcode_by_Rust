struct Solution;
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result : Vec<Vec<i32>> = vec![vec![1]];
        if num_rows > 1 {
            result.push(vec![1,1]);
        }
        for i in 2..num_rows {
            let mut tmp : Vec<i32> = result.last().unwrap().windows(2).map(|x| x[0] + x[1]).collect();
            tmp.insert(0,1);
            tmp.push(1);
            result.push(tmp);
        }
        result
    }    
}    

fn main() {
    let res = Solution::generate(5);
    println!("{:?}", res);
}