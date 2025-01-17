use tokio::process::Command;
use tokio::task;
use std::process::Stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the source and target directories
    let directories = vec![
        ("c:\\users\\hp\\big", 
         "c:\\media\\bkup\\2025_hpoptop\\users\\hp\\big"),
        // ("src_dir2", "dest_dir2"),
        // ("src_dir3", "dest_dir3"),
    ];

    // Use join handles to track tasks
    let mut tasks = Vec::new();

    for (src, dest) in directories {
        let src = src.to_string();
        let dest = dest.to_string();

        // Spawn a task for each rsync process
        let task = task::spawn(async move {
            let status = Command::new("rsync")
                .arg("-avz") // Add your desired rsync options here
                .arg(&src)
                .arg(&dest)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .await;

            match status {
                Ok(status) if status.success() => {
                    println!("Successfully synced {} to {}", src, dest);
                }
                Ok(status) => {
                    eprintln!(
                        "Failed to sync {} to {}. Exit status: {}",
                        src,
                        dest,
                        status.code().unwrap_or(-1)
                    );
                }
                Err(e) => {
                    eprintln!("Error running rsync for {} to {}: {}", src, dest, e);
                }
            }
        });

        tasks.push(task);
    }

    // Wait for all tasks to complete
    for task in tasks {
        task.await?;
    }

    Ok(())
}
