use anyhow::{Context, Result};
use chrono::prelude::*;
use chrono::DateTime;
use std::fs::{self};
use std::path::PathBuf;

const STOP_STRING: &str = "stop";
const BREAK_STRING: &str = "break";
const BREAK_INDICATOR: &str = "\\o/";
const DEFAULT_PAD: &str = " ";

pub fn start_tracking(file_path: &PathBuf, time: DateTime<Utc>) -> Result<()> {
    fs::write(file_path, time.to_rfc3339())
        .with_context(|| format!("could not write to file `{:?}`", file_path))
}

pub fn take_break(file_path: &PathBuf) -> Result<()> {
    fs::write(file_path, BREAK_STRING)
        .with_context(|| format!("couldn't write to file: `{:?}`", file_path))
}

pub fn stop_tracking(file_path: &PathBuf) -> Result<()> {
    fs::write(file_path, STOP_STRING)
        .with_context(|| format!("couldn't write to file: `{:?}`", file_path))
}

pub fn show_progress(
    file_path: &PathBuf,
    now: DateTime<Utc>,
    left_pad: Option<String>,
    right_pad: Option<String>,
) -> Result<()> {
    let status = fs::read_to_string(file_path)
        .with_context(|| format!("couldn't not read from file {:?}", file_path))?;

    if status == STOP_STRING {
        return Ok(());
    }

    let default_pad = String::from(DEFAULT_PAD);

    if status == BREAK_STRING {
        print!(
            "{}{}{}",
            left_pad.unwrap_or(default_pad.clone()),
            BREAK_STRING,
            right_pad.unwrap_or(default_pad.clone())
        );
        return Ok(());
    }

    let ts_trimmed = status.trim();

    let ts = DateTime::parse_from_rfc3339(ts_trimmed)
        .with_context(|| "couldn't not parse time from tomo's data file {:?}")?;

    let diff_seconds = now.signed_duration_since(ts.to_utc()).num_seconds();
    let chunks = diff_seconds / 150;

    if chunks >= 10 {
        print!(
            "{}{}{}",
            left_pad.unwrap_or(default_pad.clone()),
            BREAK_INDICATOR,
            right_pad.unwrap_or(default_pad.clone())
        );
        return Ok(());
    }

    let mut bar = String::new();

    for _ in 0..chunks {
        bar.push('▪')
    }
    for _ in 0..(10 - chunks) {
        bar.push('▫')
    }

    print!(
        "{}{}{}",
        left_pad.unwrap_or(default_pad.clone()),
        bar,
        right_pad.unwrap_or(default_pad.clone())
    );

    Ok(())
}
