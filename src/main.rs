use crate::csvreader::{read_csv, CsvData};

mod csvreader;

fn main() {
    let csv_data: CsvData<i32> = read_csv("input.csv").unwrap();
    println!("{:?}", csv_data);
}
