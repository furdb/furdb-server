use furdb::{FurColumn, FurDB, FurDBInfo, FurTableInfo};
use std::error::Error;
use std::path::PathBuf;

use super::request::TableGenerator;

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

pub(crate) fn generate_table_info(
    table_info_generatable: TableGenerator,
) -> Result<FurTableInfo, Box<dyn Error>> {
    let columns = table_info_generatable.columns.iter().map(|column| {
        FurColumn::new(
            &column.id,
            column.description,
            column.size,
            column.data_type,
        )
    });

    todo!();

    FurTableInfo::new(
        &table_info_generatable.name,
        table_info_generatable.converter_server.map(|s| s.as_str()),
        columns,
    )
}
