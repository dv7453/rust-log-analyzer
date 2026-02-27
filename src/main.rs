use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use colored::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// A lightweight, efficient command-line log analyzer.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the log file to analyze
    #[arg(short, long)]
    file: PathBuf,

    /// Filter logs by level
    #[arg(short, long)]
    level: Option<LogLevelFilter>,

    /// Search for a specific keyword in the log messages
    #[arg(short, long)]
    search: Option<String>,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq, Hash, Copy)]
enum LogLevelFilter {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevelFilter {
    fn colorize(&self, s: &str) -> ColoredString {
        match self {
            Self::Error => s.red().bold(),
            Self::Warn => s.yellow().bold(),
            Self::Info => s.green(),
            Self::Debug => s.blue(),
            Self::Trace => s.magenta(),
        }
    }
}

fn parse_log_level(line: &str) -> Option<LogLevelFilter> {
    // Simple fast check: check for common level strings within the line
    // In a real production tool, this might extract the level from a specific column
    let upper = line.to_uppercase();
    if upper.contains("ERROR") {
        Some(LogLevelFilter::Error)
    } else if upper.contains("WARN") {
        Some(LogLevelFilter::Warn)
    } else if upper.contains("INFO") {
        Some(LogLevelFilter::Info)
    } else if upper.contains("DEBUG") {
        Some(LogLevelFilter::Debug)
    } else if upper.contains("TRACE") {
        Some(LogLevelFilter::Trace)
    } else {
        None
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let file = File::open(&args.file)
        .with_context(|| format!("Failed to open log file at {:?}", args.file))?;
    let reader = BufReader::new(file);

    let mut level_counts: HashMap<LogLevelFilter, usize> = HashMap::new();
    let mut total_lines = 0;
    let mut matched_lines = 0;

    let search_term = args.search.as_deref().map(|s| s.to_lowercase());

    println!("{}", "Starting log analysis...".cyan().bold());

    for line_result in reader.lines() {
        let line = line_result.context("Failed to read a line from the file")?;
        total_lines += 1;

        let level = parse_log_level(&line);

        // Update statistics
        if let Some(l) = level {
            *level_counts.entry(l).or_insert(0) += 1;
        }

        // Apply level filter
        if let Some(filter_level) = &args.level {
            if Some(*filter_level) != level {
                continue;
            }
        }

        // Apply keyword search
        if let Some(ref search) = search_term {
            if !line.to_lowercase().contains(search) {
                continue;
            }
        }

        matched_lines += 1;

        // Print the matching log lines if filters are active
        if args.level.is_some() || args.search.is_some() {
            if let Some(l) = level {
                println!("{}", l.colorize(&line));
            } else {
                println!("{}", line);
            }
        }
    }

    // Summary Output
    println!("\n{}", "--- Log Analysis Summary ---".cyan().bold());
    println!("Total lines processed: {}", total_lines);
    
    if args.level.is_some() || args.search.is_some() {
        println!("Lines matching filters: {}", matched_lines);
    }

    println!("\n{}", "Log Level Counts:".bold());
    let mut counts_vec: Vec<_> = level_counts.into_iter().collect();
    // Sort by descending count
    counts_vec.sort_by(|a, b| b.1.cmp(&a.1));

    if counts_vec.is_empty() {
        println!("  No recognizable log levels found.");
    } else {
        for (level, count) in counts_vec {
            let level_str = format!("{:?}", level).to_uppercase();
            println!("  {:<8}: {}", level.colorize(&level_str), count);
        }
    }

    Ok(())
}
