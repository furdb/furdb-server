use actix_web::{App, HttpServer};

mod database;
mod table;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(database::get_info_handler)
            .service(table::get_info_handler)
            .service(table::get_data_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
