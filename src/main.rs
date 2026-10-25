use datafusion::prelude::SessionContext;
use financial_query_benchmark::benchmark::benchmark;
use financial_query_benchmark::utils::print_result;

#[tokio::main]

async fn main() {
    let ctx = SessionContext::new();
    let query = "EXPLAIN ANALYZE
SELECT ticker
FROM {file_name}";
    let result = benchmark(&ctx, query, 5, 30, "data/sharadar_sf1").await;
    print_result(&result);
}
