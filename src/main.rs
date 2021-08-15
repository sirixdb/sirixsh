mod http;
mod parsers;

use clap::Clap;
use http::{
    create_sirix, database_delete, database_info_json, read_json_resource, read_xml_resource,
    server_delete,
    sirix::{server_info, server_info_with_resources},
};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use serde_json::to_writer_pretty;
use sirix_rust_client::{
    synchronous::sirix::Sirix,
    types::{DbType, Json, MetadataType, Xml},
};
use std::{error, fmt};

use crate::{
    http::{
        database_info_xml, format_db_type, handle_error,
        types::{JsonResponse, XmlResponse},
    },
    parsers::delete::{DeleteOptsImpl, DeleteScopeTypes},
};

#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "Moshe Uminer")]
enum Commands {
    Context(parsers::ContextOpts),
    Delete(parsers::DeleteOpts),
    Read(parsers::ReadOpts),
    Info(parsers::InfoOpts),
}

impl error::Error for Commands {}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Commands::Context(opts) => write!(f, "context {:?}", opts),
            Commands::Delete(opts) => write!(f, "delete {}", opts),
            Commands::Read(opts) => write!(f, "read {}", opts),
            Commands::Info(opts) => write!(f, "info {}", opts),
        }
    }
}

fn handle_json_response(response: JsonResponse) {
    match response {
        JsonResponse::Ok(response) => {
            let _ = to_writer_pretty(std::io::stdout(), &response);
            println!("");
        }
        JsonResponse::Err(err) => handle_error(err),
    }
}

fn handle_xml_response(response: XmlResponse) {
    match response {
        XmlResponse::Ok(response) => {
            println!("{}", response);
            println!("");
        }
        XmlResponse::Err(err) => {
            handle_error(err);
        }
    }
}

fn execute_command(command: Commands, sirix: Sirix, context: &mut parsers::ContextStruct) {
    match command {
        Commands::Context(opts) => match opts.opts {
            parsers::ContextOptsImpl::Server => {
                context.context =
                    parsers::Context::Server(parsers::get_server_string(context.context.clone()));
            }
            parsers::ContextOptsImpl::Database(opts) => {
                context.context = parsers::Context::Database {
                    server: parsers::get_server_string(context.context.clone()),
                    database: opts.database,
                    db_type: match opts.db_type.as_str() {
                        "json" => DbType::Json(Json),
                        _ => DbType::XML(Xml),
                    },
                }
            }
            parsers::ContextOptsImpl::Resource(opts) => match opts.database {
                Some(database) => {
                    context.context = parsers::Context::Resource {
                        server: parsers::get_server_string(context.context.clone()),
                        database: database,
                        db_type: match opts.db_type.unwrap().as_str() {
                            "json" => DbType::Json(Json),
                            _ => DbType::XML(Xml),
                        },
                        resource: opts.resource,
                    }
                }
                None => {
                    if let parsers::Context::Database {
                        server,
                        database,
                        db_type,
                    } = context.context.clone()
                    {
                        context.context = parsers::Context::Resource {
                            server,
                            database,
                            db_type,
                            resource: opts.resource,
                        }
                    } else {
                        println!(
                                "Cannot specify resource without database except from a database context"
                            );
                    }
                }
            },
        },
        Commands::Read(opts) => {
            let metadata = match opts.metadata {
                Some(metadata) => match metadata.as_str() {
                    "all" => Some(MetadataType::All),
                    "key-and-child" => Some(MetadataType::KeyAndChild),
                    "key" => Some(MetadataType::Key),
                    _ => None,
                },
                None => None,
            };
            match opts.type_.unwrap().as_str() {
                "json" => {
                    let resource = sirix
                        .json_database(opts.database.unwrap())
                        .resource(opts.resource.unwrap());
                    let response = read_json_resource(
                        resource,
                        opts.nodekey,
                        opts.revision,
                        opts.max_depth,
                        opts.limit,
                        opts.skip,
                        metadata,
                    );
                    handle_json_response(response);
                }
                "xml" => {
                    let resource = sirix
                        .xml_database(opts.database.unwrap())
                        .resource(opts.resource.unwrap());
                    let response = read_xml_resource(
                        resource,
                        opts.nodekey,
                        opts.revision,
                        opts.max_depth,
                        opts.limit,
                        opts.skip,
                    );
                    handle_xml_response(response);
                }
                _ => panic!("Only JSON and XML supported"),
            };
        }
        Commands::Delete(opts) => {
            match opts.opts {
                DeleteOptsImpl::Scope(types) => match types {
                    DeleteScopeTypes::Context(scope) => match scope.opts {
                        parsers::DeleteContextScopesImpl::Database => {
                            // TODO - need to implement context first
                        }
                        parsers::DeleteContextScopesImpl::Resource => {
                            // TODO - need to implement context first
                        }
                        parsers::DeleteContextScopesImpl::Server => match server_delete(sirix) {
                            Ok(_) => {
                                println!("deleted all databases");
                            }
                            Err(err) => {
                                println!("failed to delete databases: {}", err);
                            }
                        },
                    },
                    DeleteScopeTypes::Explicit(scope) => match scope {
                        parsers::DeleteExplicitScope::Database { database } => {
                            match database_delete(sirix.json_database(database.clone())) {
                                Ok(_) => {
                                    println!("database {} deleted", database);
                                }
                                Err(err) => {
                                    println!("failed to delete database {}: {}", database, err);
                                }
                            }
                        }
                        parsers::DeleteExplicitScope::Resource { database, resource } => {
                            // TODO
                        }
                    },
                },
                DeleteOptsImpl::Node(opts) => {
                    // TODO
                }
            }
        }
        Commands::Info(opts) => match opts.database.to_owned() {
            // TODO: Use context here
            Some(database_name) => match opts.type_.unwrap().as_str() {
                "xml" => {
                    let database = sirix.xml_database(database_name);
                    handle_xml_response(database_info_xml(database))
                }
                _ => {
                    let database = sirix.json_database(database_name);
                    handle_json_response(database_info_json(database))
                }
            },
            None => match opts.with_resources {
                true => handle_json_response(server_info_with_resources(sirix)),
                false => handle_json_response(server_info(sirix)),
            },
        },
    }
}

fn parse(line: &std::vec::Vec<&str>, sirix: Sirix, context: &mut parsers::ContextStruct) {
    let result = Commands::try_parse_from(line);
    match result {
        Ok(command) => execute_command(command, sirix, context),
        Err(err) => {
            println!("{}", err);
        }
    };
}

fn repl() {
    let mut rl = Editor::<()>::new();
    let url = rl.readline_with_initial("url: ", ("http://localhost:9443", ""));
    let username = rl.readline_with_initial("username: ", ("admin", ""));
    let password = rl.readline_with_initial("password: ", ("admin", ""));
    let url = url.unwrap();
    let mut context = parsers::ContextStruct {
        context: parsers::Context::Server(url.clone()),
    };
    let sirix = create_sirix(&url, &username.unwrap(), &password.unwrap());
    loop {
        let prompt = match context.context.clone() {
            parsers::Context::Database {
                server: _,
                database,
                db_type,
            } => format!("{} ({}) >> ", database, format_db_type(db_type)),
            parsers::Context::Resource {
                server: _,
                database,
                db_type,
                resource,
            } => format!(
                "{}/{} ({}) >> ",
                database,
                resource,
                format_db_type(db_type)
            ),
            _ => ">> ".to_owned(),
        };
        let readline = rl.readline(prompt.as_str());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let parsed = line.split_ascii_whitespace();
                let mut collected: std::vec::Vec<&str> = parsed.collect();
                collected.insert(0, "");
                parse(&collected, sirix.clone(), &mut context);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn main() {
    repl();
}
