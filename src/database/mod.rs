use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use furdb::{FurColumn, FurDB, FurDBInfo, FurDataType, FurTable, FurTableInfo};
use std::{error::Error, path::PathBuf};

#[derive(serde::Deserialize)]
pub(crate) struct DatabaseParams {
    db_name: Option<String>,
    working_dir: Option<String>,
}

pub(crate) fn get_db(
    working_dir: Option<PathBuf>,
    db_id: &str,
    db_name: Option<String>,
) -> Result<FurDB, Box<dyn Error>> {
    let working_dir = if working_dir.is_some() {
        working_dir.unwrap()
    } else {
        PathBuf::from("D:\\Home\\Repositories\\FurDB\\TestDBs")
    };

    let mut db_path = working_dir.clone();
    db_path.push(db_id);

    let db_info = if db_name.is_some() {
        Some(FurDBInfo::new(&db_name.as_ref().unwrap())?)
    } else {
        None
    };

    FurDB::new(db_path, db_info)
}

#[derive(serde::Serialize, serde::Deserialize)]
struct DatabaseResponse {
    db_info: FurDBInfo,
    tables: Vec<String>,
}

impl DatabaseResponse {
    pub fn new(db_info: FurDBInfo, tables: Vec<String>) -> Self {
        DatabaseResponse { db_info, tables }
    }
}

#[get("/{db}")]
pub(crate) async fn database(
    path: web::Path<String>,
    req: HttpRequest,
) -> Result<impl Responder, Box<dyn Error>> {
    let db = path.into_inner();
    let params = web::Query::<DatabaseParams>::from_query(req.query_string()).unwrap();

    let working_dir = if params.working_dir.is_some() {
        Some(PathBuf::from(params.working_dir.clone().unwrap()))
    } else {
        None
    };

    let db = get_db(working_dir, &db, params.db_name.clone())?;
    let db_tables = db.get_all_table_ids()?;

    let info = db.get_info()?.clone();
    let res = DatabaseResponse::new(info, db_tables);

    Ok(web::Json(res))
}
