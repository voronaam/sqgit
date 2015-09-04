use std::process::Command;

#[macro_use]
extern crate nom;
use nom::{space, alphanumeric, IResult};

#[derive(PartialEq,Eq,Debug)]
struct Query<'a> {
    column: &'a str,
    hash: &'a str
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a query")
    }
    let string_query = &args[1];
    println!("Executing query: {}", string_query);
    let query = parse(string_query);

    println!("Parsed: {:?}", query);
    let params = build_params(&query);

    let (status, output) = rev_list(&params);
    if !status.success() {
        println!("process exited with: {}", status);
    } else {
        format_output(&output);
    }
}

fn rev_list(params: &Vec<&str>) -> (std::process::ExitStatus, String) {
    let mut git = Command::new("git");
    git.arg("rev-list").args(params);
    let output = git.output().unwrap_or_else(|e| {
        panic!("failed to execute git: {}", e)
    });
    (output.status, String::from_utf8_lossy(&output.stdout).to_string())
}
    
fn format_output(output: &String) {
    println!("--");
    println!("{}", output);
}

fn parse(input: &str) -> Query {
    named!(select_e <&[u8], Query>,
      chain!(
        tag!("SELECT") ~
        space ~
        column: map_res!(alphanumeric, std::str::from_utf8) ~
        space ~
        tag!("FROM") ~
        space ~
        hash: map_res!(alphanumeric, std::str::from_utf8),
        || {Query{column: column, hash: hash}}
      )
    );
    match select_e(input.as_bytes()) {
        IResult::Done(_, q) => q,
        _ => panic!("Failed to parse query"),
    }
}

fn build_params<'a>(query: &'a Query) -> Vec<&'a str> {
     vec!(query.hash)
}
