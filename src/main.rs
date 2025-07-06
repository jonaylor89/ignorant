//! # Ignorant
//! 
//! A phone number OSINT tool for checking if a phone number is associated with accounts
//! on various platforms (Amazon, Instagram, Snapchat).
//! 
//! This tool is designed for defensive security purposes and OSINT research to check
//! phone number exposure across platforms.
//! 
//! ## Usage
//! 
//! ```bash
//! ignorant 33 644637111
//! ```
//! 
//! ## Features
//! 
//! - **Async concurrent checking** across multiple platforms
//! - **Progress bar** with real-time updates
//! - **Colored terminal output** for better readability
//! - **Rate limit detection** and handling
//! - **Cross-platform** native binary

use std::time::Instant;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::task::JoinSet;

mod modules;
mod user_agents;

use modules::{amazon, instagram, snapchat};

/// Result of checking a phone number on a specific platform
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CheckResult {
    /// Platform name (e.g. "amazon", "instagram", "snapchat")
    pub name: String,
    /// Platform domain (e.g. "amazon.com")
    pub domain: String,
    /// Method used for checking (e.g. "login", "register", "other")
    pub method: String,
    /// Whether this platform frequently rate limits requests
    pub frequent_rate_limit: bool,
    /// Whether the current request was rate limited
    pub rate_limit: bool,
    /// Whether the phone number exists on this platform
    pub exists: bool,
}

impl CheckResult {
    /// Create a new CheckResult with default values
    fn new(name: &str, domain: &str, method: &str) -> Self {
        Self {
            name: name.to_owned(),
            domain: domain.to_owned(),
            method: method.to_owned(),
            frequent_rate_limit: false,
            rate_limit: false,
            exists: false,
        }
    }

    /// Mark this result as rate limited
    fn with_rate_limit(mut self) -> Self {
        self.rate_limit = true;
        self
    }

    /// Set whether the phone number exists on this platform
    fn with_exists(mut self, exists: bool) -> Self {
        self.exists = exists;
        self
    }
}

#[derive(Parser, Debug)]
#[command(name = "ignorant")]
#[command(about = "Check if a phone number is used on different sites")]
#[command(version = "1.2.0")]
pub struct Args {
    /// Country code of the phone (Example: 33)
    pub country_code: String,
    
    /// Target phone number (Example: 644637111)
    pub phone: String,
    
    /// Display only the sites used by the target phone number
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub only_used: bool,
    
    /// Don't color terminal output
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub no_color: bool,
    
    /// Do not clear the terminal to display the results
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub no_clear: bool,
    
    /// Set max timeout value in seconds (default: 10)
    #[arg(short = 'T', long, default_value = "10")]
    pub timeout: u64,
}

fn print_colored_text(text: &str, color: &str, no_color: bool) -> String {
    if no_color {
        text.to_owned()
    } else {
        match color {
            "green" => text.green().to_string(),
            "red" => text.red().to_string(),
            "magenta" => text.magenta().to_string(),
            _ => text.to_owned(),
        }
    }
}

async fn run_checks(
    phone: String, 
    country_code: String, 
    client: Client, 
    pb: ProgressBar
) -> Vec<CheckResult> {
    let mut join_set = JoinSet::new();
    
    // Spawn tasks for each module
    let phone_clone = phone.clone();
    let country_code_clone = country_code.clone();
    let client_clone = client.clone();
    let pb_clone = pb.clone();
    join_set.spawn(async move {
        let result = amazon::check_amazon(&phone_clone, &country_code_clone, &client_clone).await;
        pb_clone.inc(1);
        result
    });
    
    let phone_clone = phone.clone();
    let country_code_clone = country_code.clone();
    let client_clone = client.clone();
    let pb_clone = pb.clone();
    join_set.spawn(async move {
        let result = instagram::check_instagram(&phone_clone, &country_code_clone, &client_clone).await;
        pb_clone.inc(1);
        result
    });
    
    let phone_clone = phone.clone();
    let country_code_clone = country_code.clone();
    let client_clone = client.clone();
    let pb_clone = pb.clone();
    join_set.spawn(async move {
        let result = snapchat::check_snapchat(&phone_clone, &country_code_clone, &client_clone).await;
        pb_clone.inc(1);
        result
    });
    
    let mut results = Vec::new();
    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(check_result) => results.push(check_result),
            Err(e) => eprintln!("Task error: {e}"),
        }
    }
    
    results.sort_by(|a, b| a.name.cmp(&b.name));
    results
}

fn print_results(
    results: &[CheckResult],
    args: &Args,
    phone: &str,
    country_code: &str,
    start_time: Instant,
    total_modules: usize,
) {
    let description = format!(
        "{}, {}, {}",
        print_colored_text("[+] Phone number used", "green", args.no_color),
        print_colored_text("[-] Phone number not used", "magenta", args.no_color),
        print_colored_text("[x] Rate limit", "red", args.no_color)
    );
    
    let full_number = format!("+{} {}", country_code, phone);
    
    if !args.no_clear {
        print!("\x1B[2J\x1B[1;1H"); // Clear screen
    } else {
        println!();
    }
    
    println!("{}", "*".repeat(full_number.len() + 6));
    println!("   {}", full_number);
    println!("{}", "*".repeat(full_number.len() + 6));
    
    for result in results {
        if result.rate_limit && !args.only_used {
            println!("{}", print_colored_text(&format!("[x] {}", result.domain), "red", args.no_color));
        } else if !result.exists && !args.only_used {
            println!("{}", print_colored_text(&format!("[-] {}", result.domain), "magenta", args.no_color));
        } else if result.exists {
            println!("{}", print_colored_text(&format!("[+] {}", result.domain), "green", args.no_color));
        }
    }
    
    println!();
    println!("{}", description);
    println!(
        "{} websites checked in {:.2} seconds",
        total_modules,
        start_time.elapsed().as_secs_f64()
    );
}

fn print_credit() {
    println!("Twitter : @palenath");
    println!("Github : https://github.com/megadose/ignorant");
    println!("For BTC Donations : 1FHDM49QfZX6pJmhjLE5tB2K6CaTLMZpXZ");
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    print_credit();
    
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(args.timeout))
        .build()?;
    
    let total_modules = 3;
    let start_time = Instant::now();
    
    // Create progress bar
    let pb = ProgressBar::new(total_modules);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█░"),
    );
    
    let results = run_checks(
        args.phone.clone(),
        args.country_code.clone(),
        client,
        pb.clone(),
    ).await;
    
    pb.finish_and_clear();
    
    print_results(&results, &args, &args.phone, &args.country_code, start_time, total_modules as usize);
    print_credit();
    
    Ok(())
}
