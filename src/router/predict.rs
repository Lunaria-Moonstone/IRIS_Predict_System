use actix_web::{web, Responder, HttpResponse};
use rusty_machine::prelude::SupModel;
use rusty_machine::linalg::Matrix;
use crate::structure::params_struct::IrisParams;
use crate::learning::machine_learning::pretrain;

// #[post("/")]
pub async fn predicts(params: web::Json<IrisParams>) -> impl Responder {
    let lr = pretrain().unwrap();
    let matrix = Matrix::new(1, 4, Vec::from(params.predict_list));
    let predict_res = lr.predict(&matrix).unwrap();
    let res_visible: &str;
    if predict_res[0] == 0.0 {
        res_visible = "Iris-setosa";
    } else if predict_res[0] == 1.0 {
        res_visible = "Iris-versicolor";
    } else {
        res_visible = "Iris-virginica";
    }
    HttpResponse::Ok().body(format!("The predict result is: {}", res_visible))
}

