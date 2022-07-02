#[derive(Clone, serde::Deserialize)]
pub(crate) struct TableInfoGenerator {
    pub table_info: Option<TableGenerator>,
}

#[derive(Clone, serde::Deserialize)]
pub(crate) struct TableGenerator {
    pub name: String,
    pub converter_server: Option<String>,
    pub columns: Option<Vec<ColumnGenerator>>,
}

#[derive(Clone, serde::Deserialize)]
pub(crate) struct ColumnGenerator {
    pub id: String,
    pub description: Option<String>,
    pub size: u128,
    pub data_type: DataTypeGenerator,
}

#[derive(Clone, serde::Deserialize)]
pub(crate) struct DataTypeGenerator {
    pub id: String,
    pub converter_endpoint_override: Option<String>,
}
