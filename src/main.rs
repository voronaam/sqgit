use std::process::Command;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a query")
    }
    let query = &args[1];
    println!("Executing query: {}", query);
    
    let params = vec!("HEAD");
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
    println!("{}", output);
}
