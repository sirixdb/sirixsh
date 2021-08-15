use clap::Clap;

#[derive(Clap, Debug)]
pub struct DeleteOpts {
    #[clap(subcommand)]
    pub opts: DeleteOptsImpl,
}

impl std::fmt::Display for DeleteOpts {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        std::fmt::Display::fmt(&self.opts, f)
    }
}

#[derive(Clap, Debug)]
pub enum DeleteOptsImpl {
    #[clap(flatten)]
    Scope(DeleteScopeTypes),
    Node(DeleteNodeOpts),
}

impl std::fmt::Display for DeleteOptsImpl {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            DeleteOptsImpl::Node(opts) => std::fmt::Display::fmt(&opts, f),
            DeleteOptsImpl::Scope(opts) => std::fmt::Display::fmt(&opts, f),
        }
    }
}

#[derive(Clap, Debug)]
pub struct DeleteNodeOpts {
    /// nodekey of node to delete
    #[clap(index(1))]
    pub nodekey: i128,
    /// hash of node to delete
    #[clap(short, long)]
    pub etag: Option<String>,
    /// name of database - defaults to context
    #[clap(short, long)]
    pub database: Option<String>,
    /// name of resource - defaults to context
    #[clap(short, long)]
    pub resource: Option<String>,
}

impl std::fmt::Display for DeleteNodeOpts {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        let database = match &self.database {
            Some(database) => database,
            None => "<context>",
        };
        let location = match &self.resource {
            Some(resource) => database.to_owned() + "/" + resource,
            None => database.to_owned(),
        };
        match &self.etag {
            Some(etag) => write!(
                f,
                "resource: {}, nodekey: {}, etag: {}",
                location, self.nodekey, etag
            ),
            None => write!(f, "resource: {}, nodekey: {}", location, self.nodekey),
        }
    }
}

#[derive(Clap, Debug)]
pub struct DeleteContextScopes {
    #[clap(subcommand)]
    pub opts: DeleteContextScopesImpl,
}

impl std::fmt::Display for DeleteContextScopes {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        std::fmt::Display::fmt(&self.opts, f)
    }
}

#[derive(Clap, Debug)]
pub enum DeleteContextScopesImpl {
    /// delete all databases and their resources
    Server,
    /// delete the entire database in current context
    Database,
    /// delete the entire resource in current context
    Resource,
}

impl std::fmt::Display for DeleteContextScopesImpl {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            DeleteContextScopesImpl::Server => write!(f, "ServerContext"),
            DeleteContextScopesImpl::Database => write!(f, "DatabaseContext"),
            DeleteContextScopesImpl::Resource => write!(f, "ResourceContext"),
        }
    }
}

#[derive(Clap, Debug)]
pub enum DeleteExplicitScope {
    #[clap()]
    /// Delete a database by name
    Database { database: String },
    #[clap()]
    /// Delete a resource by database and resource name
    Resource { database: String, resource: String },
}

impl std::fmt::Display for DeleteExplicitScope {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        let string = match &self {
            DeleteExplicitScope::Database { database } => database.to_string(),
            DeleteExplicitScope::Resource { database, resource } => {
                database.to_owned() + "/" + resource
            }
        };
        write!(f, "{}", string)
    }
}

#[derive(Clap, Debug)]
pub enum DeleteScopeTypes {
    #[clap()]
    /// Delete an entity available in the current context
    Context(DeleteContextScopes),
    #[clap(flatten)]
    Explicit(DeleteExplicitScope),
}

impl std::fmt::Display for DeleteScopeTypes {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            DeleteScopeTypes::Context(delete_context_scope) => {
                std::fmt::Display::fmt(delete_context_scope, f)
            }
            DeleteScopeTypes::Explicit(delete_explicit_scope) => {
                std::fmt::Display::fmt(delete_explicit_scope, f)
            }
        }
    }
}
