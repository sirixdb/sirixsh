mod http;
mod parsers;

use clap::Clap;

use http::{create_sirix, database::database_info_json, read_json_resource, read_xml_resource, sirix::{server_info, server_info_with_resources}};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use serde_json::to_writer_pretty;
use sirix_rust_client::{synchronous::sirix::Sirix, types::MetadataType};

use std::{error, fmt};

use crate::http::{database_info_xml, handle_error, types::{JsonResponse, XmlResponse}};

#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "Moshe Uminer")]
enum Commands {
    Delete(parsers::DeleteOpts),
    Read(parsers::ReadOpts),
    Info(parsers::InfoOpts),
}

impl error::Error for Commands {}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
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

fn execute_command(command: Commands, sirix: Sirix) {
    match command {
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
            println!("Delete: {}", opts);
        }
        Commands::Info(opts) => match opts.database.to_owned() {
            // TODO: Use context here
            Some(database_name) => match opts.type_.unwrap().as_str() {
                "xml" => {
                    let database = sirix.xml_database(database_name);
                    handle_xml_response(database_info_xml(database))
                },
                _ => {
                    let database = sirix.json_database(database_name);
                    handle_json_response(database_info_json(database))
                }
            },
            // TODO use XML response if specified
            None => match opts.with_resources {
                Some(_) => handle_json_response(server_info_with_resources(sirix)),
                None => handle_json_response(server_info(sirix)),
            },
        },
    }
}

fn parse(line: &std::vec::Vec<&str>, sirix: Sirix) {
    let result = Commands::try_parse_from(line);
    match result {
        Ok(command) => execute_command(command, sirix),
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
    let sirix = create_sirix(&url.unwrap(), &username.unwrap(), &password.unwrap());
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let parsed = line.split_ascii_whitespace();
                let mut collected: std::vec::Vec<&str> = parsed.collect();
                collected.insert(0, "");
                parse(&collected, sirix.clone());
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
