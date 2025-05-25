// httpp - Nathanael "NateNateNate" Thevarajah
// <natenatenat3@protonmail.com> - Refer to the license for more
// information.

mod backend;
mod client;

use clap::Parser;

use client::command;

fn main() {
    let args = command::Args::parse();
    let _ = args.run();
}
