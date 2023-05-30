struct Solution;
impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut result = vec![' ';s.len()];
        for (i,&x) in indices.iter().enumerate() {
            // result[x as usize] = s.as_bytes()[i] as char; this is a second way to retrieve a character from a string
            result[x as usize] = s.chars().nth(i).unwrap();
        }
        result.iter().collect::<String>()
    }
}

fn main() {
    let res = Solution::restore_string("codeleet".to_string(),vec![4,5,6,7,0,2,1,3]);
    println!("{}",res);
}