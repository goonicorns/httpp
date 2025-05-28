// httpp - Nathanael "NateNateNate" Thevarajah
// <natenatenat3@protonmail.com> - Refer to the license for more
// information.

use crate::backend::lexer::*;
use crate::backend::parser::*;

use clap::{Parser, Subcommand, ValueEnum};

use std::fs;

/// httpp - A plain text HTTP client with .env support and zero bloat.
#[derive(Parser, Debug)]
#[command(version, about = "A better HTTP file runner", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Execute an HTTP request from a file.
    Exec(Exec),

    /// Generate new request file.
    Generate(GenerateRequestFile),
}

#[derive(Parser, Debug)]
pub struct Exec {
    /// The file to execute.
    pub file: String,

    /// The .env
    pub env: Option<String>,
}

#[derive(Parser, Debug)]
pub struct GenerateRequestFile {
    /// The output file name.
    #[arg(long)]
    pub file: String,

    /// The HTTP request type (GET, POST, etc).
    #[arg(long)]
    pub request: RequestType,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum RequestType {
    Get,
    Post,
    Patch,
    Put,
    Delete,
    Options,
}

impl Args {
    pub fn run(&self) {
        match &self.command {
            Some(Command::Exec(exec)) => {
                let contents =
                    fs::read_to_string(&exec.file).expect("Failed to read request file.");

                // If .env is provided.
                let context = if let Some(env_path) = &exec.env {
                    dotenvy::from_path_iter(env_path)
                        .expect("Failed to read .env")
                        .map(|item| item.unwrap())
                        .collect::<std::collections::HashMap<_, _>>()
                } else {
                    std::collections::HashMap::new()
                };

                let _ = execute(&contents, &context);
            }
            Some(Command::Generate(r#gen)) => {
                println!(
                    "Generating {}.httpp with method {:?}",
                    r#gen.file, r#gen.request
                );
            }
            None => {
                eprintln!("No command provided. Use --help for usage.");
            }
        }
    }
}

fn execute(file: &str, ctx: &std::collections::HashMap<String, String>) {
    let token = lex_httpp(file);
    let mut request = anal(&token).expect("Failed to parse request.");

    request.path = interpolate(&request.path, ctx);
    request.headers = request
        .headers
        .into_iter()
        .map(|(k, v)| (interpolate(&k, ctx), interpolate(&v, ctx)))
        .collect();

    if let Some(body) = request.body {
        let interpolated_body = body
            .into_iter()
            .map(|(k, v)| (interpolate(&k, ctx), interpolate(&v, ctx)))
            .collect();
        request.body = Some(interpolated_body);
    }

    println!("Parsed request (with ctx applied): {:#?}", request);
}
