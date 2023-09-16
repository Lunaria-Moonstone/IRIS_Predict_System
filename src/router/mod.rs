pub mod predict;

// use actix_web::{get, post, App, web, HttpResponse, HttpServer, Responder};

// use crate::structure;
// use crate::utils::csv_reader::read_csv_of_iris_data;

// #[get("/")]
// pub async fn home() -> impl Responder {
//     // if let Ok(_) = read_csv_of_iris_data("../data/iris.data") {
//     //     return HttpResponse::Ok().body("hello csv!");
//     // }
//     return match read_csv_of_iris_data("../data/iris.data") {
//         Ok(_) => {
//             HttpResponse::Ok().body("csv read success!")
//         },
//         Err(err) => {
//             HttpResponse::Ok().body(format!("csv read faild! the err is {}", err))
//         }
//     }
// }

// pub async fn login(user_login_info: web::Json<structure::UserLoginInfo>) -> impl Responder {
//     HttpResponse::Ok().body(format!("Welcome {}: {}", user_login_info.username, user_login_info.password))
// }