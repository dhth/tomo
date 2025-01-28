mod args;
mod common;
mod config;
mod track;
use std::ops::RangeInclusive;

use anyhow::Context;
use args::{Action, Args};
use chrono::{Duration, Utc};
use clap::Parser;
use config::DisplayConfig;
use dirs::data_dir;
use std::fs;
use std::path::PathBuf;
use track::{show_progress, start_tracking, stop_tracking, take_break};

const DATA_DIR: &str = "tomo";
const DATA_FILE: &str = ".tomo";
const ELAPSED_MINS_UPPER_LIMIT: u8 = 20;
const NUM_BLOCKS_RANGE: RangeInclusive<u8> = 3..=100;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if !(NUM_BLOCKS_RANGE).contains(&args.num_blocks) {
        return Err(anyhow::anyhow!(
            "number of blocks needs to be between 3 and 100"
        ));
    }

    let data_file_path = match args.data_file {
        Some(f) => PathBuf::from(f),
        None => {
            let user_data_dir = data_dir().unwrap_or(PathBuf::from("."));
            let data_dir = user_data_dir.join(PathBuf::from(DATA_DIR));

            if !data_dir.exists() {
                fs::create_dir_all(&data_dir).context("could not create data directory")?;
            }

            data_dir.join(PathBuf::from(DATA_FILE))
        }
    };

    let now = Utc::now();

    match args.action {
        None => {
            let config = DisplayConfig {
                pending_block: args.pending_block,
                complete_block: args.complete_block,
                left_pad: args.left_pad,
                right_pad: args.right_pad,
                delimiter: args.delimiter,
                num_blocks: args.num_blocks,
                finished_msg: args.finished_msg,
                break_msg: args.break_msg,
            };

            show_progress(&data_file_path, now, &config)
        }
        Some(Action::Start { elapsed_mins }) => {
            if elapsed_mins > ELAPSED_MINS_UPPER_LIMIT {
                return Err(anyhow::anyhow!(
                    "elapsed mins cannot be greater than {}",
                    ELAPSED_MINS_UPPER_LIMIT
                ));
            }
            start_tracking(
                &data_file_path,
                now - Duration::minutes(elapsed_mins as i64),
            )
        }
        Some(Action::Stop) => stop_tracking(&data_file_path),
        Some(Action::Break) => take_break(&data_file_path),
    }?;

    Ok(())
}
