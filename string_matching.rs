struct Solution;

impl Solution {
    pub fn string_matching (words: Vec<String>) -> Vec<String> {
        let joined = words.join(" ");
        words.into_iter().filter(|word| joined.matches(word).count()>1).collect()
    }
}

fn main() {
    let res = Solution::string_matching(vec!["mass".to_string(),"as".to_string(),"hero".to_string(),"superhero".to_string()]);
    println!("{:?}", res);
}