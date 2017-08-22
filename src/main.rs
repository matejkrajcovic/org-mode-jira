#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{ }
}
use errors::*;

mod parser;
use parser::parse_file;

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    println!("{:?}", parse_file("jira.org")?);
    Ok(())
}
