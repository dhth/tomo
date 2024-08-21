use anyhow::{Context, Result};
use chrono::prelude::*;
use chrono::DateTime;
use std::fs::{self};
use std::path::PathBuf;

const BREAK_INDICATOR: &str = "\\o/";

pub struct DisplayConfig {
    pub pending_block: String,
    pub complete_block: String,
    pub left_pad: String,
    pub right_pad: String,
    pub delimiter: String,
    pub num_blocks: u8,
}

pub fn start_tracking(file_path: &PathBuf, time: DateTime<Utc>) -> Result<()> {
    fs::write(file_path, time.to_rfc3339())
        .with_context(|| format!("could not write to file `{:?}`", file_path))
}

pub fn take_break(file_path: &PathBuf, break_string: &str) -> Result<()> {
    fs::write(file_path, break_string)
        .with_context(|| format!("couldn't write to file: `{:?}`", file_path))
}

pub fn stop_tracking(file_path: &PathBuf, stop_string: &str) -> Result<()> {
    fs::write(file_path, stop_string)
        .with_context(|| format!("couldn't write to file: `{:?}`", file_path))
}

pub fn show_progress(
    file_path: &PathBuf,
    now: DateTime<Utc>,
    break_string: &str,
    stop_string: &str,
    config: DisplayConfig,
) -> Result<()> {
    let status = fs::read_to_string(file_path)
        .with_context(|| format!("couldn't not read from file {:?}", file_path))?;

    if status == stop_string {
        return Ok(());
    }

    if status == break_string {
        print!("{}{}{}", config.left_pad, break_string, config.right_pad,);
        return Ok(());
    }

    let ts_trimmed = status.trim();

    let ts = DateTime::parse_from_rfc3339(ts_trimmed)
        .with_context(|| "couldn't not parse time from tomo's data file {:?}")?;

    let diff_seconds = now.signed_duration_since(ts.to_utc()).num_seconds();
    let chunks = diff_seconds / (25 * 60 / (config.num_blocks as i64));

    if chunks >= config.num_blocks as i64 {
        print!("{}{}{}", config.left_pad, BREAK_INDICATOR, config.right_pad,);
        return Ok(());
    }

    let mut bar = String::new();

    for _ in 0..chunks {
        bar.push_str(&config.complete_block);
        if config.delimiter.ne("") {
            bar.push_str(&config.delimiter);
        }
    }

    for _ in 0..((config.num_blocks as i64) - chunks - 1) {
        bar.push_str(&config.pending_block);
        if config.delimiter.ne("") {
            bar.push_str(&config.delimiter);
        }
    }
    bar.push_str(&config.pending_block);

    print!("{}{}{}", config.left_pad, bar, config.right_pad,);

    Ok(())
}
