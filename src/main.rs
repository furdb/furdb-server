use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use furdb::{FurColumn, FurDB, FurDBInfo, FurDataType, FurTable, FurTableInfo};
use std::{error::Error, path::PathBuf};

#[derive(serde::Deserialize)]
pub(crate) struct DatabaseParams {
    db_name: String,
}

#[get("/{db}")]
pub(crate) async fn database(
    path: web::Path<String>,
    req: HttpRequest,
) -> Result<String, Box<dyn Error>> {
    let db = path.into_inner();
    let params = web::Query::<DatabaseParams>::from_query(req.query_string()).unwrap();
    let working_dir = PathBuf::from("D:\\Home\\Repositories\\FurDB\\TestDBs");

    let mut db_path = working_dir.clone();
    db_path.push(db);

    let db_info = FurDBInfo::new(&params.db_name)?;

    Ok(format!(
        "DB {:?} | {:?} | {:?}!",
        db_path, params.db_name, db_info
    ))
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
    HttpServer::new(|| App::new().service(hello).service(database).service(table))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
