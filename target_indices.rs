struct Solution;
impl Solution {
    pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.sort();
        nums.iter().enumerate().filter(|(_,value)| *value == &target).map(|(index,_)| index as i32).collect()
    } 
}
fn main() {
    let solution = Solution::target_indices(vec![1,2,5,4,2], 2);
    println!("{:?}", solution);

}