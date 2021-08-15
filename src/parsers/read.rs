use clap::Clap;

#[derive(Clap, Debug)]
pub struct ReadOpts {
    /// The nodekey to read from the resource - defaults to the root.
    #[clap(short, long)]
    pub nodekey: Option<u128>,
    /// The maximum depth until which to read - defaults to maximum.
    #[clap(short('j'), long)]
    pub max_depth: Option<u64>,
    /// The maximum number of top level nodes to return - defaults to all.
    #[clap(short, long)]
    pub limit: Option<u64>,
    /// The last top level node to skip - defaults to none.
    #[clap(short, long)]
    pub skip: Option<u64>,
    #[clap(subcommand)]
    pub revision: Option<RevisionType>,
    /// Optionally explicity set the database to use. Requires --type.
    #[clap(long, short, requires("type"))]
    pub database: Option<String>,
    /// Optionally explicity set the resource to use. Requires --database.
    #[clap(long, short, requires("database"))]
    pub resource: Option<String>,
    /// Optionally explicitly set the type of database/resource to use.
    #[clap(long, short, possible_values = &["json", "xml"], requires("database"))]
    pub type_: Option<String>,
    /// Optionally get a metadata response instead of plain data.
    #[clap(long, short, min_values = 0, require_equals=true, possible_values = &["all", "key", "key-and-child"], default_missing_value("key-and-child"))]
    pub metadata: Option<String>,
}

// TODO handle other fields as well
impl std::fmt::Display for ReadOpts {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "nodekey: {}, maxdepth: {}, revision: {}",
            match self.nodekey {
                Some(nodekey) => nodekey.to_string(),
                None => "NULL".to_owned(),
            },
            match self.max_depth {
                Some(depth) => depth.to_string(),
                None => "NULL".to_string(),
            },
            match &self.revision {
                Some(revision) => revision.to_string(),
                None => "NULL".to_owned(),
            }
        )
    }
}

#[derive(Clap, Debug)]
pub enum RevisionType {
    /// The timestamp of a specific revision to read.
    Timestamp {
        /// The timestamp for the moment in history to read the database.
        timestamp: String,
        /// Optionally specify an <end-timestamp>, to read a series of revisions
        /// beginning with <timestamp> and ending with <end-timestamp>.
        end_timestamp: Option<String>,
    },
    /// The specific revision number to read.
    Revision {
        /// The number identifying the revision to read from.
        number: u64,
        /// Optionally specify an end-number, to read a series of revisions
        /// beginning with <number> and ending with <end-number>.
        end_number: Option<u64>,
    },
}

impl std::fmt::Display for RevisionType {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            RevisionType::Timestamp {
                timestamp,
                end_timestamp,
            } => match end_timestamp {
                Some(end_timestamp) => write!(
                    f,
                    "start-timestamp: {}, end-timestamp: {}",
                    timestamp, end_timestamp
                ),
                None => write!(f, "timestamp: {}", timestamp),
            },
            RevisionType::Revision {
                number: revision,
                end_number,
            } => match end_number {
                Some(end_number) => write!(
                    f,
                    "start-revision: {}, end-revision: {}",
                    revision, end_number
                ),
                None => write!(f, "number: {}", revision),
            },
        }
    }
}
