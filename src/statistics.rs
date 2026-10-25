use datafusion::functions_aggregate::average::avg;
use datafusion::prelude::power;
use std::ops::Sub;
use std::time::Duration;

pub fn mean(nums: &[Duration]) -> f64 {
    assert!(
        !nums.is_empty(),
        "Cannot calculate statistics on empty data"
    );
    nums.iter()
        .map(|duration| duration.as_secs_f64() * 1000.0)
        .sum::<f64>()
        / (nums.len() as f64)
}
pub fn median(nums: &[Duration]) -> f64 {
    assert!(
        !nums.is_empty(),
        "Cannot calculate statistics on empty data"
    );
    let mut values: Vec<f64> = nums
        .iter()
        .map(|duration| duration.as_secs_f64() * 1000.0)
        .collect();

    values.sort_by(|a, b| a.total_cmp(b));

    if values.len() % 2 != 0 {
        return values[values.len() / 2];
    }
    let mid = values.len() / 2;
    (values[mid - 1] + values[mid]) / 2.0
}

pub fn min(nums: &[Duration]) -> f64 {
    assert!(
        !nums.is_empty(),
        "Cannot calculate statistics on empty data"
    );
    nums.iter().min().unwrap().as_secs_f64() * 1000.0
}
pub fn max(nums: &[Duration]) -> f64 {
    assert!(
        !nums.is_empty(),
        "Cannot calculate statistics on empty data"
    );
    nums.iter().max().unwrap().as_secs_f64() * 1000.0
}
pub fn p95(nums: &[Duration]) -> f64 {
    assert!(
        !nums.is_empty(),
        "Cannot calculate statistics on empty data"
    );
    let mut values: Vec<f64> = nums
        .iter()
        .map(|duration| duration.as_secs_f64() * 1000.0)
        .collect();

    values.sort_by(|a, b| a.total_cmp(b));
    let index = (values.len() as f64 * 0.95).ceil() as usize - 1;
    values[index as usize]
}

pub fn std_dev(nums: &[Duration]) -> f64 {
    assert!(
        !nums.is_empty(),
        "Cannot calculate statistics on empty data"
    );
    let mean = mean(nums);
    let sum_squared: f64 = nums
        .iter()
        .map(|duration| {
            let millis = duration.as_secs_f64() * 1000.0;
            (millis - mean).powi(2)
        })
        .sum();
    (sum_squared / nums.len() as f64).sqrt()
}
