use crate::common::*;
use clap::{Parser, Subcommand};

/// tomo is a no-frills pomodoro progress indicator intended for tmux and similar terminal multiplexers
#[derive(Parser, Debug)]
#[command(about, long_about=None)]
pub struct Args {
    #[command(subcommand)]
    pub action: Option<Action>,
    /// String to represent a "pending" block in the progress bar
    #[arg(short = 'p', long = "pending-block", value_name = "STRING")]
    #[clap(default_value = DEFAULT_PENDING_BLOCK)]
    pub pending_block: String,
    /// String to represent a "complete" block in the progress bar
    #[arg(short = 'c', long = "complete-block", value_name = "STRING")]
    #[clap(default_value = DEFAULT_COMPLETE_BLOCK)]
    pub complete_block: String,
    /// String to pad the output with on the LHS
    #[arg(short = 'l', long = "left-pad", value_name = "STRING")]
    #[clap(default_value = DEFAULT_LEFT_PAD)]
    pub left_pad: String,
    /// String to pad the output with on the RHS
    #[arg(short = 'r', long = "right-pad", value_name = "STRING")]
    #[clap(default_value = DEFAULT_RIGHT_PAD)]
    pub right_pad: String,
    /// Delimiter between progress bar chunks
    #[arg(short = 'd', long = "delimiter", value_name = "STRING")]
    #[clap(default_value = DEFAULT_DELIMITER)]
    pub delimiter: String,
    /// Number of blocks to show in progress bar
    #[arg(short = 'n', long = "num-blocks", value_name = "NUM")]
    #[clap(default_value_t = DEFAULT_NUM_BLOCKS)]
    pub num_blocks: u8,
    /// Message to show when timer is finished
    #[arg(long = "finished-msg", value_name = "STRING")]
    #[clap(default_value = DEFAULT_FINISHED_MSG)]
    pub finished_msg: String,
    /// Message to show when on a break
    #[arg(long = "break-msg", value_name = "STRING")]
    #[clap(default_value = DEFAULT_BREAK_MSG)]
    pub break_msg: String,
    /// tomo's data file (defaults to <YOUR_DATA_DIR>/tomo/.tomo)
    #[arg(long = "data-file", value_name = "STRING", global = true)]
    pub data_file: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Action {
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
