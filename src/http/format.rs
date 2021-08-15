use sirix_rust_client::types::DbType;


pub fn format_db_type(db_type: DbType) -> String {
    match db_type {
        DbType::Json(_) => "JSON",
        DbType::XML(_) => "XML",
    }.to_string()
}