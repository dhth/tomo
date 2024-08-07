use std::process;

use chrono::Utc;
use clap::{Parser, Subcommand};
use dirs::data_dir;
use std::fs;
use std::path::PathBuf;
use tomo::{show_progress, start_tracking, stop_tracking, take_break};

const DATA_DIR: &str = "tomo";
const DATA_FILE: &str = ".tomo";

/// tomo is a no-frills pomodoro progress indicator intended for tmux and similar terminal multiplexers
#[derive(Parser, Debug)]
#[command(about, long_about=None)]
struct Args {
    #[command(subcommand)]
    action: Option<Action>,
    /// String to pad the output with on the LHS
    #[arg(long = "left-pad", value_name = "STRING")]
    left_pad: Option<String>,
    /// String to pad the output with on the RHS
    #[arg(long = "right-pad", value_name = "STRING")]
    right_pad: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
enum Action {
    /// Start a pomodoro timer
    Start,
    /// Stop timer
    Stop,
    /// Start a break
    Break,
}

fn main() {
    let args = Args::parse();

    let user_data_dir = data_dir().unwrap_or(PathBuf::from("."));
    let data_dir = user_data_dir.join(PathBuf::from(DATA_DIR));

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).unwrap_or_else(|e| {
            eprintln!("Error: could not create data directory {:?}", e);
            process::exit(1);
        });
    }

    let data_file_path = data_dir.join(PathBuf::from(DATA_FILE));

    let now = Utc::now();

    let result = match args.action {
        None => show_progress(&data_file_path, now, args.left_pad, args.right_pad),
        Some(Action::Start) => start_tracking(&data_file_path, now),
        Some(Action::Stop) => stop_tracking(&data_file_path),
        Some(Action::Break) => take_break(&data_file_path),
    };

    if let Err(e) = result {
        eprintln!("Error: {:?}", e);
        process::exit(1);
    }
}
