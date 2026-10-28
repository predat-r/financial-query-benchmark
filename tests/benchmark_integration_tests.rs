use datafusion::arrow::util::pretty::pretty_format_batches;
use datafusion::prelude::SessionContext;
use financial_query_benchmark::benchmark::benchmark;
use financial_query_benchmark::file::{convert_csv_to_parquet, load_csv, load_parquet};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn create_test_csv(dir: &Path) -> PathBuf {
    let csv_path = dir.join("financials.csv");

    fs::write(&csv_path, "ticker,revenue\nAAPL,100\nMSFT,200\nGOOG,150\n").unwrap();

    csv_path
}

async fn prepare_test_data(ctx: &SessionContext, dir: &Path) {
    let csv_path = create_test_csv(dir);
    let parquet_path = dir.join("sharadar_sf1.parquet");

    load_csv(ctx, &csv_path).await;
    convert_csv_to_parquet(ctx, &parquet_path).await;
    load_parquet(ctx, &parquet_path).await;
}

#[tokio::test]
async fn csv_and_parquet_return_equivalent_results() {
    let temp = TempDir::new().unwrap();
    let ctx = SessionContext::new();

    prepare_test_data(&ctx, temp.path()).await;

    let csv = ctx
        .sql(
            "SELECT ticker, revenue
             FROM financials_csv
             ORDER BY ticker",
        )
        .await
        .unwrap()
        .collect()
        .await
        .unwrap();

    let parquet = ctx
        .sql(
            "SELECT ticker, revenue
             FROM financials_parquet
             ORDER BY ticker",
        )
        .await
        .unwrap()
        .collect()
        .await
        .unwrap();

    assert_eq!(
        pretty_format_batches(&csv).unwrap().to_string(),
        pretty_format_batches(&parquet).unwrap().to_string()
    );
}

#[tokio::test]
async fn csv_to_parquet_conversion_produces_readable_data() {
    let temp = TempDir::new().unwrap();
    let ctx = SessionContext::new();

    prepare_test_data(&ctx, temp.path()).await;

    let batches = ctx
        .sql("SELECT * FROM financials_parquet")
        .await
        .unwrap()
        .collect()
        .await
        .unwrap();

    let row_count: usize = batches.iter().map(|batch| batch.num_rows()).sum();

    assert_eq!(row_count, 3);
}

#[tokio::test]
async fn benchmark_records_correct_number_of_measurements() {
    let temp = TempDir::new().unwrap();
    let ctx = SessionContext::new();

    let csv_path = create_test_csv(temp.path());
    let base_path = csv_path.with_extension("");

    let result = benchmark(
        &ctx,
        "SELECT ticker FROM {file_name}",
        2, // Warmups
        5, // Measured runs
        &base_path.to_str().unwrap(),
    )
    .await;

    assert_eq!(result.csv_planning_times.len(), 5);
    assert_eq!(result.parquet_planning_times.len(), 5);
    assert_eq!(result.csv_execution_times.len(), 5);
    assert_eq!(result.parquet_execution_times.len(), 5);
}
