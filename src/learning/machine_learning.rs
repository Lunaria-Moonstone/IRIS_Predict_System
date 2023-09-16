use std::error::{Error};
// use rusty_machine::linalg::{Matrix, Vector};
use rusty_machine::{learning::{lin_reg::LinRegressor, SupModel}, linalg::{Matrix, Vector}};
use crate::structure::csv_struct::Iris_Csv;
use crate::utils::csv_reader::read_csv_of_iris_data;

/**
 * map data for train
 */
fn map_data(
    dataset: Vec<Iris_Csv>, 
    vec_processed: &mut Vec<f64>,
    labels: &mut Vec<f64>
) {
    for record in dataset.iter() {
        vec_processed.push(record.sepal_width);
        vec_processed.push(record.sepal_length);
        vec_processed.push(record.petal_width);
        vec_processed.push(record.petal_length);
        if record.iris_type.eq("Iris-setosa") {
            labels.push(0.);
        } else if record.iris_type.eq("Iris-versicolor") {
            labels.push(1.);
        } else if record.iris_type.eq("Iris-virginica") {
            labels.push(2.);
        }
    }
}

/**
 * train before predict
 */
pub fn pretrain() -> Result<LinRegressor, Box<dyn Error>> {
    // read csv file

    let vec_iris_csv = read_csv_of_iris_data("data/iris.data")?;
    let length = vec_iris_csv.len();

    // load vector
    // let vec_sepal_w = Vec<f64>::new();
    // let vec_sepal_l = Vec<f64>::new();
    // let vec_petal_w = Vec<f64>::new();
    // let vec_petal_l = Vec<f64>::new();
    let mut vec_processed = Vec::<f64>::new();
    let mut labels = Vec::<f64>::new();
    // map_data(vec_iris_csv, &mut vec_sepal_w, &mut vec_sepal_l, &mut vec_petal_w, &mut vec_petal_l, &mut labels);
    map_data(vec_iris_csv, &mut vec_processed, &mut labels);
    let matrix = Matrix::new(length, 4, vec_processed);
    let targets = Vector::new(labels);

    let mut lr = LinRegressor::default();
    lr.train(&matrix, &targets)?;

    Ok(lr)
} 