use datafusion::dataframe::DataFrame;
use datafusion::prelude::SessionContext;
use std::cmp::min;
use std::time::Duration;

pub fn print_result(results: BenchmarkingResult) {
    let csv_times = results.csv_execution_times;
    let parquet_times = results.parquet_execution_times;
    let runs = min(parquet_times.len(), csv_times.len());
    for i in 0..runs {
        println!(
            "Run: {i} Csv: {:#?}, Parquet:{:#?} ",
            csv_times[i],
            parquet_times[i]
        );
    }
}
pub async fn exec_query(ctx: &SessionContext, query: &str, file_type: FileType) -> DataFrame {
    let table_name = match file_type {
        FileType::Csv => "financials_csv",
        FileType::Parquet => "financials_parquet",
    };

    let sql = query.replace("{file_name}", table_name);

    ctx.sql(&sql).await.unwrap()
}

pub struct BenchmarkingResult {
    pub csv_execution_times: Vec<Duration>,
    pub parquet_execution_times: Vec<Duration>,
}

impl BenchmarkingResult {
    pub fn new() -> Self {
        Self {
            csv_execution_times: Vec::new(),
            parquet_execution_times: Vec::new(),
        }
    }
}

pub enum FileType {
    Csv,
    Parquet,
}
