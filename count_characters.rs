struct Solution;

impl Solution {
    pub fn count_characters(words: Vec<String> , chars : String ) -> i32 {
    // The value 97 represents the ASCII value of the lowercase letter 'a'. In ASCII encoding, the lowercase letters 'a' to 'z' have consecutive values from 97 to 122.
    // 122 - 97 + 1 = 26    
    // By subtracting 97 from the ASCII value of a lowercase letter, we can calculate the relative position or index of that letter in an array. For example, if u represents the byte value of the lowercase letter 'a', then u as usize - 97 would evaluate to 0, representing the first index of the array.
        let mut map = [0;26];
        chars.bytes().for_each(|c| map[c as usize - 97] +=1);

        words.iter().fold(0, |acc, word| {
            let mut map_word = map;
            for u in word.bytes() {
                map_word[u as usize - 97] -= 1;
                if map_word[u as usize - 97] < 0 {
                    return acc
                }
            };
            acc + word.len() as i32
        })
        
    }
}

fn main() {
    let res = Solution::count_characters(vec!["hello".to_string(),"world".to_string(),"leetcode".to_string()],"welldonehoneyr".to_string());
    println!("{}", res);
}