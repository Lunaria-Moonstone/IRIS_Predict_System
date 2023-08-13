pub mod router;
pub mod structure;

use actix_web::{HttpServer, App, web, guard};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                router::home
            )
            .service(
                web::resource("/")
                    .route(web::post().to(router::login))
            )
    })
    .bind(("127.0.0.1", 8091))?
    .run().await
}