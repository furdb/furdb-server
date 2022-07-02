use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod database;
mod table;

// #[get("/{db}/{tb}")]
// pub(crate) async fn table(path: web::Path<(String, String)>) -> std::io::Result<String> {
//     let (db, tb) = path.into_inner();
//     Ok(format!("DB {} TB {}!", db, tb))
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(database::get_info_handler)
            .service(table::get_info_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
