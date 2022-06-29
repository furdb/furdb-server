use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use furdb::{FurColumn, FurDB, FurDBInfo, FurDataType, FurTable, FurTableInfo};
use std::{path::PathBuf, error::Error};

#[get("/{db}")]
pub(crate) async fn database(path: web::Path<String>, db_name: String) -> Result<String, Box<dyn Error>> {
    let db = path.into_inner();
    let WORKING_DIR = PathBuf::from("D:\\Home\\Repositories\\FurDB\\TestDBs");

    let db_path = WORKING_DIR.clone().push(db);
    let db_info = FurDBInfo::new(&db_name)?;

    Ok(format!("DB {:?}!", db_path))
}

#[get("/{db}/{tb}")]
pub(crate) async fn table(path: web::Path<(String, String)>) -> std::io::Result<String> {
    let (db, tb) = path.into_inner();
    Ok(format!("DB {} TB {}!", db, tb))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(database)
            .service(table)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}