use polars::prelude::*;
use polars::df;
use std::fs::File;

fn test_polars_lazy() -> Result<String> {
    // scan a parquet file lazily
    LazyFrame::scan_ipc(path: String, stop_after_n_rows: Option<usize>, cache: bool);
    let lf: LazyFrame = LazyFrame::scan_parquet("./test.parquet".into(), None, true);
    println!("{:?}", lf);

    return Ok(String::from(""));
}

fn test_polars() -> Result<String> {

    let file = File::open("./test.parquet").unwrap();

    let df = ParquetReader::new(file).finish()?;
    println!("{:?}", df);

    return Ok(String::from(""));
}

fn main() {
    test_polars();
    test_polars_lazy();
}
