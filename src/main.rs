mod args;
mod config;
mod generator;
mod checker;
mod validator;
mod entropy;
mod error;

use args::{Cli, Commands};
use config::{Config, CharMask, parse_parameters};
use clap::Parser;
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("\x1b[31;1mError:\x1b[0m {}", e);
        process::exit(1);
    }
}

fn run() -> error::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate(options) => {
            let length = options.length.unwrap_or(12);
            // let repeat_allowed = parse_repeat(options.repeat.as_deref())?;
            let params_str = options.parameters.as_deref().unwrap_or("lbns"); // дефолтная маска, если не указана
            let mask = parse_parameters(params_str)?;

            let config = Config::new(length, options.repeat, mask);
            let password = generator::generate_password(&config)?;
            
            println!("{}", password);
        }
        Commands::Check { password, options } => {
            // let repeat_allowed = parse_repeat(options.repeat.as_deref())?;
            let params_str = options.parameters.as_deref().unwrap_or("");
            let mask = parse_parameters(params_str)?;

            let config = if options.length.is_some() || options.repeat.is_some() || options.parameters.is_some() {
                let len_req = options.length.unwrap_or(0);
                Some(Config::new(len_req, options.repeat, mask))
            } else {
                None
            };

            let result = checker::check_password(&password, config.as_ref());
            
            println!("Password: {}", result.password);
            println!("Length: {}", result.length);
            println!("Classes: {}", format_mask(result.present_classes));
            println!("Unique characters: {}", result.unique);
            println!("Real Entropy: {:.2} bits", result.entropy);
            
            if config.is_some() {
                println!("Meets minimum requirements: {}", result.meets_requirements);
            }
        }
    }
    Ok(())
}

fn format_mask(mask: CharMask) -> String {
    let mut s = String::new();
    if mask.contains(CharMask::LATIN) { s.push('l'); }
    if mask.contains(CharMask::CYRILLIC) { s.push('c'); }
    if mask.contains(CharMask::CAPITAL) { s.push('b'); }
    if mask.contains(CharMask::DIGIT) { s.push('n'); }
    if mask.contains(CharMask::SPECIAL) { s.push('s'); }
    if s.is_empty() { s.push_str("none"); }
    s
}