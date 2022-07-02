use actix_web::{get, web, HttpRequest, Responder};
use std::{error::Error, path::PathBuf};

mod utils;
use utils::{generate_table_info, get_db};

mod response;
use response::TableResponse;

mod params;
use params::TableParams;

mod request;
use request::TableInfoGenerator;

#[get("/{db}/{tb}")]
pub(crate) async fn get_info_handler(
    path: web::Path<(String, String)>,
    req: HttpRequest,
    table_info_generatable: web::Json<TableInfoGenerator>,
) -> Result<impl Responder, Box<dyn Error>> {
    let (db, tb) = path.into_inner();
    let params = web::Query::<TableParams>::from_query(req.query_string()).unwrap();

    let working_dir = params.working_dir.as_ref().map(|wd| PathBuf::from(wd));

    let db = get_db(working_dir, &db, params.db_name.clone())?;

    let table_info = table_info_generatable
        .table_info
        .as_ref()
        .map(|table_info_generatable| generate_table_info(table_info_generatable.clone()).unwrap());

    let tb = db.get_table(&tb, table_info)?;

    let info = tb.get_info()?.clone();
    let res = TableResponse::new(info);

    Ok(web::Json(res))
}
