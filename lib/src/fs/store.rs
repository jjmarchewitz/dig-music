use polars::prelude::*;
use std::fs::File;
use std::path::PathBuf;

fn remove_columns_with_container_dtype(df: &DataFrame) -> PolarsResult<DataFrame> {
    let cols_to_write_to_csv: Vec<&str> = df
        .get_columns()
        .into_iter()
        .filter_map(|c| {
            if c._dtype().is_primitive() {
                Some(c.name())
            } else {
                None
            }
        })
        .collect();

    df.select(cols_to_write_to_csv)
}

pub fn write_df_to_csv(df: &mut DataFrame, csv_path: &PathBuf) -> PolarsResult<()> {
    let mut df_with_primitive_columns_only = remove_columns_with_container_dtype(df)?;

    let mut file = File::create(csv_path).expect("could not create CSV file");

    CsvWriter::new(&mut file)
        .include_header(true)
        .with_separator(b',')
        .finish(&mut df_with_primitive_columns_only)?;

    Ok(())
}
