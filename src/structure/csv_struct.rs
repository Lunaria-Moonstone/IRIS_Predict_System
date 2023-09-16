use serde::Deserialize;

#[derive(Deserialize)]
pub struct Iris_Csv {
    pub sepal_length: f64,
    pub sepal_width: f64,
    pub petal_length: f64,
    pub petal_width: f64,
    pub iris_type: String
}