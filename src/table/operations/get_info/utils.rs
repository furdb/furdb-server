use furdb::{FurColumn, FurDB, FurDBInfo, FurDataType, FurTableInfo};
use std::error::Error;
use std::path::PathBuf;

use super::request::{ColumnGenerator, DataTypeGenerator, TableGenerator};

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
    let columns = table_info_generatable.columns.map(|column_generators| {
        column_generators
            .iter()
            .map(|column_generatable| generate_column(column_generatable.clone()).unwrap())
            .collect()
    });

    FurTableInfo::new(
        &table_info_generatable.name,
        table_info_generatable.converter_server.map(|s| s.as_str()),
        columns,
    )
}

pub(crate) fn generate_column(
    column_generatable: ColumnGenerator,
) -> Result<FurColumn, Box<dyn Error>> {
    FurColumn::new(
        &column_generatable.id,
        column_generatable.description.map(|s| s.as_str()),
        column_generatable.size,
        generate_data_type(column_generatable.data_type)?,
    )
}

pub(crate) fn generate_data_type(
    data_type_generatable: DataTypeGenerator,
) -> Result<FurDataType, Box<dyn Error>> {
    FurDataType::new(
        &data_type_generatable.id,
        data_type_generatable
            .converter_endpoint_override
            .map(|s| s.as_str()),
    )
}
