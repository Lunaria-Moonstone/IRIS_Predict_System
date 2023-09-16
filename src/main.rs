pub mod router;
pub mod structure;
pub mod utils;
pub mod learning;

use actix_web::{HttpServer, App, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/iris/predict")
                    .route(web::post().to(router::predict::predicts))
            )
    })
    .bind(("127.0.0.1", 8091))?
    .run().await
}