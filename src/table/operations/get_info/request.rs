#[derive(serde::Deserialize)]
pub(crate) struct TableGenerator {
    pub name: String,
    pub converter_server: Option<String>,
    pub columns: Vec<String>,
}

#[derive(serde::Deserialize)]
pub(crate) struct ColumnGenerator {
    pub id: String,
    pub description: String,
    pub size: u128,
    pub data_type: DataTypeGenerator,
}

#[derive(serde::Deserialize)]
pub(crate) struct DataTypeGenerator {
    pub id: String,
    pub converter_endpoint_override: Option<String>,
}
