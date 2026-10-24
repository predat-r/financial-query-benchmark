use std::path::Path;
use std::time::Duration;
use datafusion::arrow::array::RecordBatch;
use datafusion::dataframe::DataFrame;
use datafusion::prelude::SessionContext;
use crate::file::{convert_csv_to_parquet, load_csv, load_parquet};

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
