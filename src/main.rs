use datafusion::arrow::record_batch::RecordBatch;
use datafusion::prelude::{DataFrame, SessionContext};
use financial_query_benchmark::benchmark::{get_results, prepare_for_benchmarking};
use financial_query_benchmark::file::{convert_csv_to_parquet, load_csv, load_parquet};
use financial_query_benchmark::utils::{exec_query, print_result};
use std::path::Path;
use std::time::Duration;

#[tokio::main]

async fn main() {
    let ctx = SessionContext::new();
    prepare_for_benchmarking(&ctx).await;

    //csv query
    let df = exec_query(
        &ctx,
        "SELECT ticker, calendardate, revenue
        FROM {csv}
        ORDER BY revenue DESC
        LIMIT 10",
    )
    .await;
    let (duration, results) = get_results(df).await;
    print_result(&results, duration, "csv");

    //parquet query
    let df = exec_query(
        &ctx,
        "SELECT ticker, calendardate, revenue
        FROM {parquet}
        ORDER BY revenue DESC
        LIMIT 10",
    )
    .await;
    let (duration, results) = get_results(df).await;
    print_result(&results, duration, "parquet");
}


