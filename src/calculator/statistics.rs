use std::collections::HashMap;
use crate::calculator::arithmetic;

pub fn mean(nums: &[f64]) -> f64 {
    let sum = arithmetic::add(nums);
    sum / (nums.len() as f64)
}

pub fn median(nums: &[i32]) -> f64 {
    let len = nums.len();
    let mid = len / 2;
    if len % 2 == 0 {
        mean(&nums.into_iter().map(|n| *n as f64).collect::<Vec<f64>>()[(mid - 1)..(mid + 1)])
    } else {
        f64::from(nums[mid])
    }
}

pub fn mode(nums: &[i32]) -> f64 {
    let mut occurrences: HashMap<&i32, i32> = HashMap::new();

    for entry in nums {
        *occurrences.entry(entry).or_insert(0) += 1;
    }

    *occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .unwrap()
        .0 as f64
}
