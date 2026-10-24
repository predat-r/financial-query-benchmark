use datafusion::prelude::SessionContext;
use financial_query_benchmark::benchmark::benchmark;
use financial_query_benchmark::utils::print_result;

#[tokio::main]

async fn main() {
    let ctx = SessionContext::new();
    let query = "SELECT ticker, calendardate, revenue
            FROM {file_name}
                WHERE revenue > 100000000000
            ORDER BY revenue DESC
            LIMIT 10";
    let result = benchmark(&ctx, query, 5, 30).await;
    print_result(&result);
}
