use std::collections::HashMap;
use crate::calculator::arithmetic;

pub fn mean(nums: &[i32]) {
    println!("{}", mean_average(nums));
}

fn mean_average(nums: &[i32]) -> f64 {
    let sum: i32 = arithmetic::addition(nums);
    f64::from(sum) / (nums.len() as f64)
}

pub fn median(nums: &[i32]) {
    println!("{}", median_average(nums));
}

fn median_average(nums: &[i32]) -> f64 {
    let len = nums.len();
    let mid = len / 2;
    if len % 2 == 0 {
        mean_average(&nums[(mid - 1)..(mid + 1)])
    } else {
        f64::from(nums[mid])
    }
}

pub fn mode(nums: &[i32]) {
    println!("{}", mode_average(nums));
}

fn mode_average(nums: &[i32]) -> i32 {
    let mut occurrences: HashMap<&i32, i32> = HashMap::new();
    let mut max: (i32, i32) = (0, 0);

    for entry in nums {
        let count = occurrences.entry(entry).or_insert(0);
        *count += 1;
    }

    for (&&key, &val) in &occurrences {
        if val > max.1 {
            max = (key, val);
        }
    }

    max.0
}
