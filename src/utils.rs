use datafusion::arrow::array::RecordBatch;
use datafusion::arrow::util::pretty::print_batches;
use std::time::Duration;
use datafusion::dataframe::DataFrame;
use datafusion::prelude::SessionContext;

pub fn print_result(results: &[RecordBatch], duration: Duration, file_type: &str) {
    print_batches(results).unwrap();
    println!("Time taken: {:?} by {file_type}", duration,);
}
pub async fn exec_query(ctx: &SessionContext, query: &str) -> DataFrame {
    ctx.sql(
        query
            .replace("{csv}", "financials_csv")
            .replace("{parquet}", "financials_parquet")
            .as_str(),
    )
    .await
    .unwrap()
}
