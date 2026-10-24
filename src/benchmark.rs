use crate::file::{convert_csv_to_parquet, load_csv, load_parquet};
use crate::utils::{BenchmarkingResult, FileType};
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
    let mut result = BenchmarkingResult::default();
    prepare_for_benchmarking(ctx).await;
    for _i in 0..warmup_runs {
        let (_planning_duration, df) = exec_query(ctx, query, FileType::Csv).await;
        get_results(df).await;
        let (_planning_duration, df) = exec_query(ctx, query, FileType::Parquet).await;
        get_results(df).await;
    }
    for _i in 0..measured_runs {
        let (planning_duration, df) = exec_query(ctx, query, FileType::Csv).await;
        result.csv_planning_times.push(planning_duration);
        let (execution_duration, _results) = get_results(df).await;
        result.csv_execution_times.push(execution_duration);
        let (planning_duration, df) = exec_query(ctx, query, FileType::Parquet).await;
        result.parquet_planning_times.push(planning_duration);
        let (execution_duration, _results) = get_results(df).await;
        result.parquet_execution_times.push(execution_duration);
    }
    result
}
pub async fn prepare_for_benchmarking(ctx: &SessionContext) {
    load_csv(ctx).await;
    if !Path::new("data/sharadar_sf1.parquet").exists() {
        convert_csv_to_parquet(ctx).await;
    }
    load_parquet(ctx).await;
}
pub async fn get_results(df: DataFrame) -> (Duration, Vec<RecordBatch>) {
    let start = std::time::Instant::now();
    let results = df.collect().await.unwrap();
    (start.elapsed(), results)
}

pub async fn exec_query(
    ctx: &SessionContext,
    query: &str,
    file_type: FileType,
) -> (Duration, DataFrame) {
    let table_name = match file_type {
        FileType::Csv => "financials_csv",
        FileType::Parquet => "financials_parquet",
    };
    let start = std::time::Instant::now();

    let sql = query.replace("{file_name}", table_name);
    let df = ctx.sql(&sql).await.unwrap();

    (start.elapsed(), df)
}
