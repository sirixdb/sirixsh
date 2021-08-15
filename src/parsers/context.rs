use clap::Clap;
use sirix_rust_client::types::DbType;

pub struct ContextStruct {
    pub context: Context,
}

#[derive(Debug, Clone)]
pub enum Context {
    Server(String),
    Database {
        server: String,
        database: String,
        db_type: DbType,
    },
    Resource {
        server: String,
        database: String,
        db_type: DbType,
        resource: String,
    },
}

pub fn get_server_string(context: Context) -> String {
    match context {
        Context::Server(server) => server,
        Context::Database {server, ..} => server,
        Context::Resource {server, ..} => server,
    }
}

#[derive(Clap, Debug)]
pub struct ContextOpts {
    #[clap(subcommand)]
    pub opts: ContextOptsImpl,
}

#[derive(Clap, Debug)]
#[clap(flatten)]
pub enum ContextOptsImpl {
    Server,
    Database(DatabaseOpts),
    DatabaseAndResource(DatabaseAndResourceOpts),
    Resource(ResourceOpts),
}

#[derive(Clap, Debug)]
pub struct DatabaseOpts {
    #[clap(index(1))]
    pub database: String,
    #[clap(index(2), possible_values = &["json", "xml"], requires("database"))]
    pub db_type: String,
}

#[derive(Clap, Debug)]
pub struct DatabaseAndResourceOpts {
    #[clap(index(1))]
    pub database: String,
    #[clap(index(2), possible_values = &["json", "xml"], requires("database"))]
    pub db_type: String,
    #[clap(index(3))]
    pub resource: String,
}

#[derive(Clap, Debug)]
pub struct ResourceOpts {
    #[clap(index(1))]
    pub resource: String,
}
