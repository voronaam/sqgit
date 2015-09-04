use std::process::Command;

fn main() {
    let status = rev_list("--count");
    println!("process exited with: {}", status);
}

fn rev_list(params: &str) -> std::process::ExitStatus {
    let command = format!("git ref-list {}", params);   
    println!("{}", command);
    let mut git = Command::new(command);
    let status = git.status().unwrap_or_else(|e| {
        panic!("failed to execute git: {}", e)
    });
    status
}
    
