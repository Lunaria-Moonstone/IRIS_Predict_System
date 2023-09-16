use std::fs::File;
use std::io::Read;
// use serde::Deserialize;
use crate::structure::csv_struct::Iris_Csv;

/*
* read csv file and return tuple
*/
pub fn read_csv_of_iris_data(pth: &str) -> Result<Vec<Iris_Csv>, csv::Error> {
    // read file
    let mut csv_file = File::open(pth).unwrap();
    let mut csv_comment: String = String::new();
    csv_file.read_to_string(&mut csv_comment).unwrap();

    // write file comment into structure 'Iris_Csv'
    let mut res_vec: Vec<Iris_Csv> = Vec::new();
    let mut csv_reader = csv::Reader::from_reader(csv_comment.as_bytes());
    for record in csv_reader.deserialize() {
        let record: Iris_Csv = record?;
        println!(
            "the sepal width is {}, length is {}; the petal width is {}, length is {}; it's {}.",
            record.sepal_width, 
            record.sepal_length,
            record.petal_width,
            record.petal_length,
            record.iris_type
        );
        res_vec.push(record);
    }
    Ok(res_vec)
} 