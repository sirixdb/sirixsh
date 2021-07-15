use clap::Clap;

#[derive(Clap, Debug)]
pub struct InfoOpts {
    /// Read database info for the entire server.
    #[clap(short, long)]
    pub server: Option<bool>,
    /// Include resource info when reading info for the entire server.
    #[clap(short, long, requires("server"))]
    pub with_resources: Option<String>,
    /// Read info for database.
    #[clap(short, long, conflicts_with("server"))]
    pub database: Option<String>,
    /// Optionally explicitly set the response format.
    #[clap(long, short, possible_values = &["json", "xml"])]
    pub type_: Option<String>,
}

impl std::fmt::Display for InfoOpts {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self.database.to_owned() {
            Some(database) => write!(f, "database: {}", database),
            None => {
                match self.with_resources {
                    Some(_) => write!(f, "server with_resources"),
                    None => write!(f, "server")
                }
            }
        }
    }
}