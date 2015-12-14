use std::process::Command;

#[macro_use]
extern crate nom;
use nom::{space, alphanumeric, digit, IResult};

#[derive(PartialEq,Eq,Debug)]
struct Query<'a> {
    column: String,
    hash: String,
    limit: Option<&'a str>,
    offset: Option<&'a str>,
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

fn rev_list(params: &Vec<String>) -> (std::process::ExitStatus, String) {
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
    named!(limit_e <&[u8], &str>,  chain!(space ~ tag!("LIMIT")  ~ space ~ l: map_res!(digit, std::str::from_utf8), || {l}));
    named!(offset_e <&[u8], &str>, chain!(space ~ tag!("OFFSET") ~ space ~ l: map_res!(digit, std::str::from_utf8), || {l}));

    named!(select_e <&[u8], Query>,
      chain!(
        tag!("SELECT") ~
        space ~
        column: map_res!(alphanumeric, std::str::from_utf8) ~
        space ~
        tag!("FROM") ~
        space ~
        hash: map_res!(alphanumeric, std::str::from_utf8) ~
        limit: opt!(complete!(limit_e)) ~
        offset: opt!(complete!(offset_e))
        , || {Query{
            column: column.to_string(),
            hash: hash.to_string(),
            limit: limit,
            offset: offset}}
      )
    );

    let parsed = select_e(input.as_bytes());
    match parsed {
        IResult::Done(_, q) => q,
        _ => panic!("Failed to parse query: {:?}", parsed),
    }
}

fn build_params(query: &Query) -> Vec<String> {
     let mut res = vec!(query.hash.to_string());
     if let Some(x) = query.limit {
         res.push(format!("--max-count={}", x));
     }
     if let Some(x) = query.offset {
         res.push(format!("--skip={}", x));
     }
     res
}
