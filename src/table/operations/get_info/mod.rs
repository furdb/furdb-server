use actix_web::{get, web, HttpRequest, Responder};
use std::{error::Error, path::PathBuf};

mod utils;
use utils::{generate_table_info, get_db};

mod response;
use response::TableResponse;

mod params;
use params::TableParams;

mod request;
use request::TableGenerator;

#[get("/{db}/{tb}")]
pub(crate) async fn get_info_handler(
    path: web::Path<(String, String)>,
    req: HttpRequest,
    table_info_generatable: web::Json<TableGenerator>,
) -> Result<impl Responder, Box<dyn Error>> {
    let (db, tb) = path.into_inner();
    let params = web::Query::<TableParams>::from_query(req.query_string()).unwrap();

    let working_dir = if params.working_dir.is_some() {
        Some(PathBuf::from(params.working_dir.clone().unwrap()))
    } else {
        None
    };

    let db = get_db(working_dir, &db, params.db_name.clone())?;

    let table_info = generate_table_info(table_info_generatable.0)?;

    let tb = db.get_table(&tb, Some(table_info))?;

    let info = tb.get_info()?.clone();
    let res = TableResponse::new(info);

    Ok(web::Json(res))
}
