use datafusion::arrow::record_batch::RecordBatch;
use datafusion::arrow::util::pretty::print_batches;
use datafusion::dataframe::DataFrameWriteOptions;
use datafusion::prelude::{CsvReadOptions, DataFrame, ParquetReadOptions, SessionContext};
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
    print_result(&results, duration,"csv");

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
    print_result(&results, duration,"parquet");
}

async fn prepare_for_benchmarking(ctx: &SessionContext) {
    load_csv(ctx).await;
    if (!Path::new("data/sharadar_sf1.parquet").exists()) {
        convert_csv_to_parquet(ctx).await;
    }
    load_parquet(ctx).await;
}

async fn load_parquet(ctx: &SessionContext) {
    ctx.register_parquet(
        "financials_parquet",
        "data/sharadar_sf1.parquet",
        ParquetReadOptions::new(),
    )
    .await
    .expect("failed to register parquet");
}

async fn load_csv(ctx: &SessionContext) {
    ctx.register_csv(
        "financials_csv",
        "data/sharadar_sf1.csv",
        CsvReadOptions::new(),
    )
    .await
    .expect("failed to register csv");
}

async fn convert_csv_to_parquet(ctx: &SessionContext) {
    let query = ctx
        .sql("SELECT * FROM financials_csv")
        .await
        .expect("query failed");
    query
        .write_parquet(
            "data/sharadar_sf1.parquet",
            DataFrameWriteOptions::new(),
            None, // can also specify parquet writing options here
        )
        .await
        .expect("Unable to write to parquet");
}

fn print_result(results: &[RecordBatch], duration: Duration, file_type: &str) {
    print_batches(results).unwrap();
    println!("Time taken: {:?} by {file_type}", duration,);
}

async fn exec_query(ctx: &SessionContext, query: &str) -> DataFrame {
    ctx.sql(
        query
            .replace("{csv}", "financials_csv")
            .replace("{parquet}", "financials_parquet")
            .as_str(),
    )
    .await
    .unwrap()
}

async fn get_results(df: DataFrame) -> (Duration, Vec<RecordBatch>) {
    let start = std::time::Instant::now();
    let results = df.collect().await.unwrap();
    (start.elapsed(), results)
}
