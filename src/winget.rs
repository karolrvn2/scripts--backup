use std::process::Command;

fn main() {
    let output = Command::new("winget")
        .arg("--version")
        .output()
        .expect("Failed to execute winget command");

    if output.status.success() {
        let version = String::from_utf8_lossy(&output.stdout);
        println!("Winget version: {}", version);
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
