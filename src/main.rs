use actix_web::{
    HttpServer,
    App
};

mod api;
use api::notes_api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(notes_api::get_notes)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
