use datafusion::prelude::{CsvReadOptions, SessionContext};

#[tokio::main]

async fn main() {
    let ctx = SessionContext::new();
    ctx.register_csv("financials", "data/sharadar_sf1.csv", CsvReadOptions::new())
        .await
        .expect("Unable to open example.csv");
    let query = ctx
        .sql(
            "
    SELECT ticker, calendardate, revenue
    FROM financials
    ORDER BY revenue DESC
    LIMIT 10
",
        )
        .await
        .unwrap();
    let results = query.collect().await.unwrap();
    datafusion::arrow::util::pretty::print_batches(&results).unwrap();
}
