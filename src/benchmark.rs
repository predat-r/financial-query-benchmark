use crate::file::{convert_csv_to_parquet, load_csv, load_parquet};
use crate::utils::{BenchmarkingResult, FileType, exec_query};
use datafusion::arrow::array::RecordBatch;
use datafusion::dataframe::DataFrame;
use datafusion::prelude::SessionContext;
use std::path::Path;
use std::time::Duration;

pub async fn benchmark(
    ctx: &SessionContext,
    query: &str,
    warmup_runs: usize,
    measured_runs: usize,
) -> BenchmarkingResult {
    let mut Result = BenchmarkingResult::new();
    prepare_for_benchmarking(ctx).await;
    for i in 0..warmup_runs {
        let df = exec_query(ctx, query, FileType::Csv).await;
        let (duration, _results) = get_results(df).await;
        Result.csv_execution_times.push(duration);
        let df = exec_query(ctx, query, FileType::Parquet).await;
        let (duration, _results) = get_results(df).await;
        Result.parquet_execution_times.push(duration);
    }
    for i in 0..measured_runs {
        let df = exec_query(ctx, query, FileType::Csv).await;
        let (duration, results) = get_results(df).await;
        Result.csv_execution_times.push(duration);
        let df = exec_query(ctx, query, FileType::Parquet).await;
        let (duration, results) = get_results(df).await;
        Result.parquet_execution_times.push(duration);
    }
    Result
}
pub async fn prepare_for_benchmarking(ctx: &SessionContext) {
    load_csv(ctx).await;
    if (!Path::new("data/sharadar_sf1.parquet").exists()) {
        convert_csv_to_parquet(ctx).await;
    }
    load_parquet(ctx).await;
}
pub async fn get_results(df: DataFrame) -> (Duration, Vec<RecordBatch>) {
    let start = std::time::Instant::now();
    let results = df.collect().await.unwrap();
    (start.elapsed(), results)
}
