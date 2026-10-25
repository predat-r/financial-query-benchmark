use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

const INPUT: &str = "data/sharadar_sf1.csv";
const OUTPUT: &str = "data/sharadar_sf1_large.csv";
const TARGET_ROWS: usize = 1_000_000;

fn main() -> std::io::Result<()> {
    let input = File::open(INPUT)?;
    let reader = BufReader::new(input);

    let mut lines = reader.lines();

    // Read the CSV header.
    let header = lines.next().expect("CSV file is empty")?;

    // Load the original data rows.
    let rows: Vec<String> = lines.collect::<Result<_, _>>()?;

    assert!(!rows.is_empty(), "CSV contains no data rows");

    let output = File::create(OUTPUT)?;
    let mut writer = BufWriter::new(output);

    // Write the header once.
    writeln!(writer, "{header}")?;

    // Repeat the original rows until we reach 1 million.
    for row in rows.iter().cycle().take(TARGET_ROWS) {
        writeln!(writer, "{row}")?;
    }

    writer.flush()?;

    println!("Generated {TARGET_ROWS} rows");
    println!("Output: {OUTPUT}");

    Ok(())
}