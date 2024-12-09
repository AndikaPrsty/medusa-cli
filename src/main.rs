use std::{fs, io};
use std::process::Command;

fn get_username() -> Result<String, String> {
    // Try to get the username using the appropriate command based on the OS
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg("echo %USERNAME%")
            .output()
    } else {
        Command::new("whoami").output()
    };

    match output {
        Ok(output) if output.status.success() => {
            let username = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(username)
        }
        _ => Err("Failed to get the username".to_string()),
    }
}

fn git_clone(username: &String) {
    // Remove repository if it exists
    let home_dir = format!("/Users/{}", username);
    let project_dir = format!("{home_dir}/Desktop/project-trashcan", home_dir = home_dir);
    let repo_name = "toleh-app-api";
    let repo_path = format!("{dir}/{repo}", dir = project_dir, repo = repo_name);
    if let Err(e) = fs::remove_dir_all(&repo_path) {
        if e.kind() != std::io::ErrorKind::NotFound {
            eprintln!("Failed to remove repository: {}", e);
            return;
        }
    }

    // Change node version to v14
    let change_node_version_command = format!("source {}/.nvm/nvm.sh && nvm use 14", home_dir);

    // Clone the repository
    if let Err(e) = Command::new("git")
        .current_dir(&project_dir)
        .arg("clone")
        .arg(format!(
            "https://github.com/AndikaPrsty/{repo}.git",
            repo = repo_name
        ))
        .status() {
            eprintln!("Failed to clone repository: {}", e);
            std::process::exit(1)
        }

    // Install the dependencies
    if let Err(e) = Command::new("sh")
        .arg("-c")
        .current_dir(&repo_path)
        .arg(format!("{} && npm install", change_node_version_command))
        .status() {
            eprintln!("Failed to install dependencies: {}", e);
            std::process::exit(1)
        }
}


fn main() {
    let mut is_first_open = true;
    loop {
        if is_first_open {
            println!("Welcome to medusa-cli!");
            println!("What would you like to do?");
        }

        println!("1. Clone the Medusa repository");
        println!("2. Exit");
        println!("(Type the number to select an option)");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error input");

        is_first_open = false;

        match input.trim() {
            "1" => 
            match get_username() {
                Ok(username) => git_clone(&username),
                Err(e) => eprintln!("{}", e),
            },
            "2" => {
                println!("Exiting...");
                break;
            },
            _ =>  {
                println!("Invalid option selected.");
                println!("Available options");
                continue;
            }
        }
    }
}
