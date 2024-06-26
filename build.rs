use std::process::Command;

fn main() {
    set_commit_info();
}

fn set_commit_info() {
    let output = match Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--date=short")
        .arg("--format=%H %h %cd")
        .output()
    {
        Ok(output) if output.status.success() => output,
        _ => return,
    };
    let stdout = String::from_utf8(output.stdout).unwrap();
    let mut parts = stdout.split_whitespace();
    let mut next = || parts.next().unwrap();
    println!("cargo:rustc-env=COMMIT_HASH={}", next());
    println!("cargo:rustc-env=COMMIT_SHORT_HASH={}", next());
    println!("cargo:rustc-env=COMMIT_DATE={}", next())
}
