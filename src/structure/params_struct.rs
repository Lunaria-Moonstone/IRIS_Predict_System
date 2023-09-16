use serde::Deserialize;

#[derive(Deserialize)]
pub struct IrisParams {
    pub predict_list: [f64; 4]
}