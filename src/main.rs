use datafusion::dataframe::DataFrameWriteOptions;
use datafusion::prelude::{CsvReadOptions, ParquetReadOptions, SessionContext};

#[tokio::main]

async fn main() {
    let ctx = SessionContext::new();

    //csv queries
    ctx.register_csv(
        "financials_csv",
        "data/sharadar_sf1.csv",
        CsvReadOptions::new(),
    )
    .await
    .expect("failed to register csv");
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

    let query = ctx
        .sql(
            "
        SELECT ticker, calendardate, revenue
        FROM financials_csv
        ORDER BY revenue DESC
        LIMIT 10
    ",
        )
        .await
        .unwrap();
    let results = query.collect().await.unwrap();
    datafusion::arrow::util::pretty::print_batches(&results).unwrap();

    //parquet query
    ctx.register_parquet(
        "financials_parquet",
        "data/sharadar_sf1.parquet",
        ParquetReadOptions::new(),
    )
    .await
    .expect("failed to register parquet");

    let query = ctx
        .sql(
            "
        SELECT ticker, calendardate, revenue
        FROM financials_parquet
        ORDER BY revenue DESC
        LIMIT 10
    ",
        )
        .await
        .unwrap();
    let results = query.collect().await.unwrap();
    datafusion::arrow::util::pretty::print_batches(&results).unwrap();
}
