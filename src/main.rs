use std::process;

use chrono::{Duration, Utc};
use clap::{Parser, Subcommand};
use dirs::data_dir;
use std::fs;
use std::path::PathBuf;

const DATA_DIR: &str = "tomo";
const DATA_FILE: &str = ".tomo";
const ELAPSED_MINS_UPPER_LIMIT: u8 = 20;

/// tomo is a no-frills pomodoro progress indicator intended for tmux and similar terminal multiplexers
#[derive(Parser, Debug)]
#[command(about, long_about=None)]
struct Args {
    #[command(subcommand)]
    action: Option<Action>,
    /// String to represent a "pending" block in the progress bar
    #[arg(short = 'p', long = "pending-block", value_name = "STRING")]
    #[clap(default_value = tomo::DEFAULT_PENDING_BLOCK)]
    pending_block: String,
    /// String to represent a "complete" block in the progress bar
    #[arg(short = 'c', long = "complete-block", value_name = "STRING")]
    #[clap(default_value = tomo::DEFAULT_COMPLETE_BLOCK)]
    complete_block: String,
    /// String to pad the output with on the LHS
    #[arg(short = 'l', long = "left-pad", value_name = "STRING")]
    #[clap(default_value = tomo::DEFAULT_LEFT_PAD)]
    left_pad: String,
    /// String to pad the output with on the RHS
    #[arg(short = 'r', long = "right-pad", value_name = "STRING")]
    #[clap(default_value = tomo::DEFAULT_RIGHT_PAD)]
    right_pad: String,
    /// Delimiter between progress bar chunks
    #[arg(short = 'd', long = "delimiter", value_name = "STRING")]
    #[clap(default_value = tomo::DEFAULT_DELIMITER)]
    delimiter: String,
    /// Number of blocks to show in progress bar
    #[arg(short = 'n', long = "num-blocks", value_name = "NUM")]
    #[clap(default_value_t = tomo::DEFAULT_NUM_BLOCKS)]
    num_blocks: u8,
    /// Message to show when timer is finished
    #[arg(long = "finished-msg", value_name = "STRING")]
    #[clap(default_value = tomo::DEFAULT_FINISHED_MSG)]
    finished_msg: String,
    /// Message to show when on a break
    #[arg(long = "break-msg", value_name = "STRING")]
    #[clap(default_value = tomo::DEFAULT_BREAK_MSG)]
    break_msg: String,
}

#[derive(Subcommand, Debug, Clone)]
enum Action {
    /// Start a pomodoro timer
    Start {
        /// Start tracking with n minutes already elapsed
        #[arg(short = 'e', long = "elapsed-mins", value_name = "NUM")]
        #[clap(default_value = "0")]
        elapsed_mins: u8,
    },
    /// Stop timer
    Stop,
    /// Start a break
    Break,
}

fn main() {
    let args = Args::parse();

    if !(3..=100).contains(&args.num_blocks) {
        eprintln!("Error: number of blocks needs to be between 3 and 100");
        process::exit(1);
    }

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
        None => {
            let config = tomo::DisplayConfig {
                pending_block: args.pending_block,
                complete_block: args.complete_block,
                left_pad: args.left_pad,
                right_pad: args.right_pad,
                delimiter: args.delimiter,
                num_blocks: args.num_blocks,
                finished_msg: args.finished_msg,
                break_msg: args.break_msg,
            };

            tomo::show_progress(&data_file_path, now, &config)
        }
        Some(Action::Start { elapsed_mins }) => {
            if elapsed_mins > ELAPSED_MINS_UPPER_LIMIT {
                eprintln!(
                    "Error: elapsed mins cannot be greater than {}",
                    ELAPSED_MINS_UPPER_LIMIT
                );
                process::exit(1);
            }
            tomo::start_tracking(
                &data_file_path,
                now - Duration::minutes(elapsed_mins as i64),
            )
        }
        Some(Action::Stop) => tomo::stop_tracking(&data_file_path),
        Some(Action::Break) => tomo::take_break(&data_file_path),
    };

    if let Err(e) = result {
        eprintln!("Error: {:?}", e);
        process::exit(1);
    }
}
