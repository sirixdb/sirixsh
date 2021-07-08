mod http;
mod parsers;

use clap::Clap;

use http::{read_json_resource, read_xml_resource, create_sirix};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use serde_json::to_writer_pretty;
use sirix_rust_client::synchronous::sirix::Sirix;

use std::{error, fmt};

use crate::http::{handle_error, types::{JsonResponse, XmlResponse}};

#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "Moshe Uminer")]
enum Commands {
    Delete(parsers::delete::DeleteOpts),
    Read(parsers::read::ReadOpts),
}

impl error::Error for Commands {}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Commands::Delete(opts) => write!(f, "delete {}", opts),
            Commands::Read(opts) => write!(f, "read {}", opts),
        }
    }
}

fn execute_command(command: Commands, sirix: Sirix) {
    match command {
        Commands::Read(opts) => {
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
                    );
                    match response {
                        JsonResponse::Ok(response) => {
                            let _ = to_writer_pretty(std::io::stdout(), &response);
                            println!("");
                        }
                        JsonResponse::Err(err) => handle_error(err),
                    }
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
                _ => panic!("Only JSON and XML supported"),
            };
        }
        Commands::Delete(opts) => {
            println!("Delete: {}", opts);
        }
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
