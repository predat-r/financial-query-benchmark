use crate::statistics::{max, mean, median, min, p95, std_dev};
use comfy_table::Table;
use comfy_table::presets::UTF8_FULL;
use datafusion::functions_nested::map::map;
use std::collections::HashMap;
use std::time::Duration;

pub fn print_result(results: &BenchmarkingResult) {
    print_times(&results);
    print_statistics(&results);
}

fn print_statistics(results: &BenchmarkingResult) {
    let mut table = Table::new();
    table.load_style(UTF8_FULL);

    table.set_header(vec![
        "Statistic",
        "CSV Planning",
        "Parquet Planning",
        "CSV Execution",
        "Parquet Execution",
    ]);

    let statistics = ["Mean", "Median", "Min", "Max", "p95", "Std Dev"];

    for stat in statistics {
        table.add_row(vec![
            stat.to_string(),
            format!(
                "{:.3} ms",
                get_statistic(&results.csv_planning_times, stat).unwrap()
            ),
            format!(
                "{:.3} ms",
                get_statistic(&results.parquet_planning_times, stat).unwrap()
            ),
            format!(
                "{:.3} ms",
                get_statistic(&results.csv_execution_times, stat).unwrap()
            ),
            format!(
                "{:.3} ms",
                get_statistic(&results.parquet_execution_times, stat).unwrap()
            ),
        ]);
    }

    println!("{table}");
}

fn get_statistic(nums: &[Duration], stat: &str) -> Result<f64, &'static str> {
    match stat {
        "Mean" => Ok(mean(nums)),
        "Median" => Ok(median(nums)),
        "Min" => Ok(min(nums)),
        "Max" => Ok(max(nums)),
        "p95" => Ok(p95(nums)),
        "Std Dev" => Ok(std_dev(nums)),
        _ => Err("Invalid Statistic"),
    }
}
fn print_times(results: &BenchmarkingResult) {
    let mut table = Table::new();
    table.load_style(UTF8_FULL);

    table.set_header(vec![
        "Run",
        "CSV Planning",
        "Parquet Planning",
        "CSV Execution",
        "Parquet Execution",
    ]);

    let runs = [
        results.csv_planning_times.len(),
        results.parquet_planning_times.len(),
        results.csv_execution_times.len(),
        results.parquet_execution_times.len(),
    ]
    .into_iter()
    .min()
    .unwrap_or(0);

    for i in 0..runs {
        table.add_row(vec![
            i.to_string(),
            format!(
                "{:.3} ms",
                results.csv_planning_times[i].as_secs_f64() * 1000.0
            ),
            format!(
                "{:.3} ms",
                results.parquet_planning_times[i].as_secs_f64() * 1000.0
            ),
            format!(
                "{:.3} ms",
                results.csv_execution_times[i].as_secs_f64() * 1000.0
            ),
            format!(
                "{:.3} ms",
                results.parquet_execution_times[i].as_secs_f64() * 1000.0
            ),
        ]);
    }

    println!("{table}");
}
#[derive(Default)]
pub struct BenchmarkingResult {
    pub csv_execution_times: Vec<Duration>,
    pub parquet_execution_times: Vec<Duration>,
    pub csv_planning_times: Vec<Duration>,
    pub parquet_planning_times: Vec<Duration>,
}

pub enum FileType {
    Csv,
    Parquet,
}
