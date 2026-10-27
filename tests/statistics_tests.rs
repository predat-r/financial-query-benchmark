use financial_query_benchmark::statistics::{max, mean, median, min, p95, std_dev};
use std::time::Duration;
mod tests {
    use super::*;
    use std::time::Duration;

    fn durations(values: &[u64]) -> Vec<Duration> {
        values.iter().map(|&ms| Duration::from_millis(ms)).collect()
    }

    fn assert_approx_eq(actual: f64, expected: f64) {
        let tolerance = 1e-9;

        assert!(
            (actual - expected).abs() < tolerance,
            "Expected {expected}, got {actual}"
        );
    }

    // MEAN

    #[test]
    fn mean_of_multiple_values() {
        let nums = durations(&[10, 20, 30, 40]);
        assert_approx_eq(mean(&nums), 25.0);
    }

    #[test]
    fn mean_of_single_value() {
        let nums = durations(&[42]);
        assert_approx_eq(mean(&nums), 42.0);
    }

    #[test]
    fn mean_with_fractional_milliseconds() {
        let nums = [Duration::from_micros(1500), Duration::from_micros(2500)];

        assert_approx_eq(mean(&nums), 2.0);
    }

    // MEDIAN

    #[test]
    fn median_of_odd_length() {
        let nums = durations(&[30, 10, 50, 20, 40]);
        assert_approx_eq(median(&nums), 30.0);
    }

    #[test]
    fn median_of_even_length() {
        let nums = durations(&[40, 10, 30, 20]);
        assert_approx_eq(median(&nums), 25.0);
    }

    #[test]
    fn median_of_single_value() {
        let nums = durations(&[15]);
        assert_approx_eq(median(&nums), 15.0);
    }

    // MINIMUM

    #[test]
    fn min_of_unsorted_values() {
        let nums = durations(&[30, 5, 20, 10]);
        assert_approx_eq(min(&nums), 5.0);
    }

    #[test]
    fn min_of_single_value() {
        let nums = durations(&[7]);
        assert_approx_eq(min(&nums), 7.0);
    }

    // MAXIMUM

    #[test]
    fn max_of_unsorted_values() {
        let nums = durations(&[30, 5, 50, 10]);
        assert_approx_eq(max(&nums), 50.0);
    }

    #[test]
    fn max_of_single_value() {
        let nums = durations(&[7]);
        assert_approx_eq(max(&nums), 7.0);
    }

    // P95 — NEAREST RANK

    #[test]
    fn p95_of_twenty_values() {
        let nums = durations(&(1..=20).collect::<Vec<u64>>());

        // ceil(20 * 0.95) = 19
        assert_approx_eq(p95(&nums), 19.0);
    }

    #[test]
    fn p95_of_thirty_values() {
        let nums = durations(&(1..=30).collect::<Vec<u64>>());

        // ceil(30 * 0.95) = 29
        assert_approx_eq(p95(&nums), 29.0);
    }

    #[test]
    fn p95_of_small_dataset() {
        let nums = durations(&[10, 20, 30, 40, 50]);

        // ceil(5 * 0.95) = 5
        assert_approx_eq(p95(&nums), 50.0);
    }

    #[test]
    fn p95_of_unsorted_values() {
        let nums = durations(&[50, 10, 40, 20, 30]);
        assert_approx_eq(p95(&nums), 50.0);
    }

    // STANDARD DEVIATION — POPULATION

    #[test]
    fn std_dev_of_identical_values() {
        let nums = durations(&[10, 10, 10, 10]);
        assert_approx_eq(std_dev(&nums), 0.0);
    }

    #[test]
    fn std_dev_of_known_values() {
        let nums = durations(&[2, 4, 4, 4, 5, 5, 7, 9]);

        // Mean = 5, population variance = 4
        // Standard deviation = sqrt(4) = 2
        assert_approx_eq(std_dev(&nums), 2.0);
    }

    #[test]
    fn std_dev_of_single_value() {
        let nums = durations(&[100]);
        assert_approx_eq(std_dev(&nums), 0.0);
    }

    // EMPTY INPUT

    #[test]
    fn all_statistics_reject_empty_input() {
        let nums: Vec<Duration> = vec![];

        for statistic in [
            mean as fn(&[Duration]) -> f64,
            median,
            min,
            max,
            p95,
            std_dev,
        ] {
            assert!(
                std::panic::catch_unwind(|| statistic(&nums)).is_err(),
                "Statistic should panic on empty input"
            );
        }
    }
}
