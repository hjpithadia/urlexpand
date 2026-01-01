use std::io::{self, Write};
use std::time::Duration;
use urlexpand::{is_shortened, unshorten_blocking};

fn main() {
    println!("URL Expander (type 'help' for commands)\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();
        let cmd = parts.first().map(|s| *s).unwrap_or("");
        let url = parts.get(1).map(|s| *s).unwrap_or("");

        match cmd {
            "check" | "c" => {
                if url.is_empty() {
                    println!("usage: check <url>");
                } else if is_shortened(url) {
                    println!("✓ shortened");
                } else {
                    println!("✗ not shortened");
                }
            }
            "expand" | "e" => {
                if url.is_empty() {
                    println!("usage: expand <url>");
                } else if !is_shortened(url) {
                    println!("✗ not a shortened url");
                } else {
                    match unshorten_blocking(url, Some(Duration::from_secs(10))) {
                        Ok(expanded) => println!("→ {}", expanded),
                        Err(e) => println!("✗ {}", e),
                    }
                }
            }
            "help" | "h" => {
                println!("check <url>  - check if url is shortened");
                println!("expand <url> - expand shortened url");
                println!("quit         - exit");
            }
            "quit" | "q" | "exit" => break,
            "" => {}
            _ => println!("unknown command (try 'help')"),
        }
    }
}

