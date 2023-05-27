use std::collections::BinaryHeap;
struct Solution;
impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut heap = BinaryHeap::new();
        for (i,point) in score.iter().enumerate() {
            heap.push((point,i))
        }
        let mut result = vec![String::new();score.len()];
        let mut current = 1;
        while let Some((_,i)) = heap.pop() {
            let placement = match current {
                1 => "Gold Medal".to_string(),
                2 => "Silver Medal".to_string(),
                3 => "Bronze Medal".to_string(),
                n => n.to_string(),
            };
            result[i] = placement;
            current +=1;
        }
        result         
    }
}

fn main() {
    let res = Solution::find_relative_ranks(vec![10,3,6,9,8]);
    println!("{:?}",res);
}