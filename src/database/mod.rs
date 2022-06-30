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

    let db = FurDB::new(db_path, Some(db_info))?;

    let db_tables = db.get_all_table_ids()?;

    Ok(format!("{:?}\n{:?}", db, db_tables))
}
