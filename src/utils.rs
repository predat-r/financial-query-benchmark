use comfy_table::Table;
use comfy_table::presets::UTF8_FULL;
use std::time::Duration;

pub fn print_result(results: &BenchmarkingResult) {
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
