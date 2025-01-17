use tokio::process::Command;
use tokio::task;
use std::process::Stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the source and target directories
    let src = "c:/users/hp/";
    let dst = "c:/media/bkup/2025_hpoptop/users/hp/";
    let directories = vec![
        // ("r/tools")
        ("r"),
        ("Pictures"),
        // ("Documents/big") // skip
        ("Documents"), // moved to ~/big - whole point is to NOT have to have gazilion of manual exclusions / inclusions - fs first
        // ("Documents/now"),
        // ("Documents/setup"),
        // ("Documents/ShareX"),
        
        // "remote:2025_hpoptop/users/hp/big"),
        // "/media/bkup/2025_hpoptop/users/hp/r/tools"),
        // ("c:\\users\\hp\\r/big", 
        //   "/media/bkup/2025_hpoptop/users/hp/r/big"),
        // ("c:\\users\\hp\\r/rust", 
        //   "/media/bkup/2025_hpoptop/users/hp/r/rust"),

        //   ("c:\\users\\hp\\documents", 
        //   "/media/bkup/2025_hpoptop/users/hp/documents"),

        // ("c:\\users\\hp\\documents\\big", 
        // // "remote:2025_hpoptop/users/hp/big"),
        // "/media/bkup/2025_hpoptop/users/hp/documents/big"),



        // ("src_dir2", "remote:dest_dir2"),
        // ("src_dir3", "remote:dest_dir3"),
    ];

    // Use join handles to track tasks
    let mut tasks = Vec::new();

    for (src_item) in directories {
        let src = src.to_owned() + "/" + src_item;
        let dest = dst.to_owned() + "/"  + src_item;

        // Spawn a task for each rclone sync process
        let task = task::spawn(async move {
            let status = Command::new("C:\\Users\\HP\\Documents\\big\\setup\\software\\rclone-v1.69.0-windows-amd64\\rclone.exe")
                .arg("sync") // Use rclone's sync command
                .arg(&src)
                .arg(&dest)
                .arg("--progress") // Add your desired rclone options here
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
                    eprintln!("Error running rclone for {} to {}: {}", src, dest, e);
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
