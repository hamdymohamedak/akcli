use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::env;

// ANSI escape codes for text colors
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

fn main() {
    println!("{}CMD [1] OR POWERSHELL [2]!{}", GREEN, RESET);
    let mut terminal_kind = String::new();
    io::stdin().read_line(&mut terminal_kind).expect("Failed to read line");
    let terminal_kind = terminal_kind.trim(); // Trim newline character

    let terminal_value = if terminal_kind == "1" {
        "cmd.exe"
    } else if terminal_kind == "2" {
        "powershell.exe"
    } else {
        panic!("Invalid choice, please enter 1 or 2.");
    };

    println!(
        "{}You can find the GitHub Repo from here https://github.com/hamdymohamedak/akcli{}",
        GREEN, RESET
    );
    println!("");

    println!("Enter the path for the text file where commands will be saved:");
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path).expect("Failed to read line");
    let file_path = file_path.trim(); // Trim newline character

    let mut current_dir = env::current_dir().unwrap();

    let mut executed_commands: HashSet<String> = HashSet::new();

    loop {
        // Take user input
        print!("{}AKCLI:>>>{} ", GREEN, RESET);
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed before input
        let mut text_input = String::new();
        io::stdin()
            .read_line(&mut text_input)
            .expect("Failed to read line");

        // Remove trailing newline character
        let trimmed_input = text_input.trim().to_string();

        // Check if the user wants to exit
        if trimmed_input == "exit" {
            println!("{}Goodbye!{}", GREEN, RESET);
            break;
        }

        // Change directory if the input starts with "cd "
        if trimmed_input.starts_with("cd ") {
            let new_dir = &trimmed_input[3..].trim();
            if new_dir.is_empty() {
                eprintln!("Invalid directory.");
                continue;
            }
            match env::set_current_dir(new_dir) {
                Ok(_) => {
                    current_dir = env::current_dir().unwrap();
                    println!("{}", current_dir.display());
                }
                Err(e) => eprintln!("Failed to change directory: {}", e),
            }
            continue;
        }

        // Execute command with user input
        let output = Command::new(terminal_value)
            .args(["/C", &trimmed_input])
            .output()
            .expect("Failed to execute process");

        // Print the output to the user
        println!(
            "{}{}:\n{}",
            RED,
            RESET,
            String::from_utf8_lossy(&output.stdout)
        );

        // Print error message if there is any
        if !output.stderr.is_empty() {
            println!(
                "{}Error: {}{}",
                RED,
                String::from_utf8_lossy(&output.stderr),
                RESET
            );
        }

        // Store command in the specified text file if it's unique and valid
        if output.status.success() && !executed_commands.contains(&trimmed_input) {
            executed_commands.insert(trimmed_input.clone());
            if let Err(err) = store_command(&trimmed_input, file_path) {
                eprintln!("Failed to store command: {}", err);
            }
        }
    }
}

fn store_command(command: &str, file_path: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)?;
    writeln!(file, "{}", command)?;
    Ok(())
}
