use std::process::Command;

fn main() {
    let params = vec!("--count", "HEAD");
    let (status, output) = rev_list(&params);
    println!("process exited with: {}", status);
    println!("{}", output);
}

fn rev_list(params: &Vec<&str>) -> (std::process::ExitStatus, String) {
    let mut git = Command::new("git");
    git.arg("rev-list").args(params);
    let output = git.output().unwrap_or_else(|e| {
        panic!("failed to execute git: {}", e)
    });
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    //(output.status, String::from_utf8_lossy(&output.stdout))
    (output.status, stdout)
}
    
