pub fn mean(nums: &[i32]) {
    println!("{}", mean_average(nums));
}

fn mean_average(nums: &[i32]) -> f64 {
    let sum: i32 = Iterator::sum(nums.iter());
    f64::from(sum) / (nums.len() as f64)
}
