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
        Context::Database { server, .. } => server,
        Context::Resource { server, .. } => server,
    }
}

#[derive(Clap, Debug)]
pub struct ContextOpts {
    #[clap(subcommand)]
    pub opts: ContextOptsImpl,
}

#[derive(Clap, Debug)]
pub enum ContextOptsImpl {
    Server,
    Database(DatabaseOpts),
    Resource(ResourceOpts),
}

#[derive(Clap, Debug)]
pub struct DatabaseOpts {
    pub database: String,
    #[clap(possible_values = &["json", "xml"])]
    pub db_type: String,
}

#[derive(Clap, Debug)]
pub struct ResourceOpts {
    #[clap(long, short, requires("database"))]
    pub database: Option<String>,
    #[clap(long("type"), short('t'), requires("database"), possible_values = &["json", "xml"])]
    pub db_type: Option<String>,
    pub resource: String,
}
