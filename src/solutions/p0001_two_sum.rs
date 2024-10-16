// 1. Two Sum: https://leetcode.com/problems/two-sum

use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Map to store numbers and their indices
        let mut numbers_to_index_map = HashMap::new();

        // Iterate through nums array
        for (current_index, &current_number) in nums.iter().enumerate() {
            // Calculate the complement needed to reach target
            let required_number = target - current_number;

            // If found in map, return the pair of indices
            if let Some(&required_index) = numbers_to_index_map.get(&required_number) {
                return vec![required_index as i32, current_index as i32];
            }

            // Otherwise, store the current number with its index
            numbers_to_index_map
                .entry(current_number)
                .or_insert(current_index);
        }

        // Return empty array if no solution
        vec![]
    }
}
