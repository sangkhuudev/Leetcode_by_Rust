struct Solution;

impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut item_map = std::collections::HashMap::new();

        // Merge similar items from items1
        for item in items1 {
            let key = item[0];
            let value = item[1];
            *item_map.entry(key).or_insert(0) += value;
        }

        // Merge similar items from items2
        for item in items2 {
            let key = item[0];
            let value = item[1];
            *item_map.entry(key).or_insert(0) += value;
        }

        // Convert the item_map back to a vector of items
        for (key, value) in item_map {
            result.push(vec![key, value]);
        }

        // Sort the result by the key in ascending order
        result.sort_by_key(|row| row[0]);
        result
    }
}

fn main() {
    let res = Solution::merge_similar_items(vec![vec![3, 1], vec![2, 3], vec![5, 6]], vec![vec![3, 4], vec![1, 3], vec![5, 2]]);
    println!("{:?}", res); // [[1, 3], [2, 3], [3, 5], [5, 8]]
}
