use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::process::Command;

// ANSI escape codes for text colors
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

fn main() {
    println!("{}CMD [1] OR POWERSHELL [2]!{}", GREEN, RESET);
    let mut terminal_kind = String::new();
    io::stdin().read_line(&mut terminal_kind).expect("Failed to read line");
    let terminal_kind = terminal_kind.trim(); // Trim newline character

    let mut terminal_value = ""; // Initialize terminal_value with a default value
    if terminal_kind == "1" {
        terminal_value = "cmd";
    } else if terminal_kind == "2" {
        terminal_value = "powershell";
    }

    println!(
        "{}You can find the GitHub Repo from here https://github.com/hamdymohamedak/akcli{}",
        GREEN, RESET
    );
    println!("");

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

        // Store command in a text file if it's unique and valid
        if output.status.success() && !executed_commands.contains(&trimmed_input) {
            executed_commands.insert(trimmed_input.clone());
            if let Err(err) = store_command(&trimmed_input) {
                eprintln!("Failed to store command: {}", err);
            }
        }
    }
}

fn store_command(command: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("CommandLine_Storage.txt")?;
    writeln!(file, "{}", command)?;
    Ok(())
}
