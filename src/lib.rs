use anyhow::{Context, Result};
use chrono::prelude::*;
use chrono::DateTime;
use std::fs::{self};
use std::path::PathBuf;

pub const DEFAULT_PENDING_BLOCK: &str = "▫";
pub const DEFAULT_COMPLETE_BLOCK: &str = "▪";
pub const DEFAULT_LEFT_PAD: &str = " ";
pub const DEFAULT_RIGHT_PAD: &str = " ";
pub const DEFAULT_DELIMITER: &str = "";
pub const DEFAULT_NUM_BLOCKS: u8 = 10;

pub const BREAK_STRING: &str = "break";
pub const STOP_STRING: &str = "stop";

pub const BREAK_INDICATOR: &str = "\\o/";

pub struct DisplayConfig {
    pub pending_block: String,
    pub complete_block: String,
    pub left_pad: String,
    pub right_pad: String,
    pub delimiter: String,
    pub num_blocks: u8,
}

impl Default for DisplayConfig {
    fn default() -> Self {
        DisplayConfig {
            pending_block: DEFAULT_PENDING_BLOCK.to_string(),
            complete_block: DEFAULT_COMPLETE_BLOCK.to_string(),
            left_pad: DEFAULT_LEFT_PAD.to_string(),
            right_pad: DEFAULT_RIGHT_PAD.to_string(),
            delimiter: DEFAULT_DELIMITER.to_string(),
            num_blocks: DEFAULT_NUM_BLOCKS,
        }
    }
}

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

pub fn show_progress(file_path: &PathBuf, now: DateTime<Utc>, config: DisplayConfig) -> Result<()> {
    let status = fs::read_to_string(file_path)
        .with_context(|| format!("couldn't not read from file {:?}", file_path))?;

    if status == STOP_STRING {
        return Ok(());
    }

    if status == BREAK_STRING {
        print!("{}{}{}", config.left_pad, BREAK_STRING, config.right_pad,);
        return Ok(());
    }

    let ts_trimmed = status.trim();

    let ts = DateTime::parse_from_rfc3339(ts_trimmed)
        .with_context(|| "couldn't not parse time from tomo's data file {:?}")?;

    let diff_seconds = now.signed_duration_since(ts.to_utc()).num_seconds();
    let output = get_progress_bar(diff_seconds, config);
    println!("{}", output);

    Ok(())
}

pub fn get_progress_bar(diff_seconds: i64, config: DisplayConfig) -> String {
    let mut chunks = diff_seconds / (25 * 60 / (config.num_blocks as i64));

    if chunks >= config.num_blocks as i64 {
        chunks = config.num_blocks as i64
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

    format!("{}{}{}", config.left_pad, bar, config.right_pad,)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_progress_works_with_defaults() {
        // GIVEN
        let config = DisplayConfig::default();

        // WHEN
        let got = get_progress_bar(10 * 60, config);

        // THEN
        let expected = String::from(" ▪▪▪▪▫▫▫▫▫▫ ");

        assert_eq!(got, expected);
    }

    #[test]
    fn get_progress_respects_custom_padding() {
        // GIVEN
        let default_config = DisplayConfig::default();
        let config = DisplayConfig {
            left_pad: String::from("["),
            right_pad: String::from("]"),
            ..default_config
        };

        // WHEN
        let got = get_progress_bar(0, config);

        // THEN
        let expected = String::from("[▫▫▫▫▫▫▫▫▫▫]");

        assert_eq!(got, expected);
    }

    #[test]
    fn get_progress_respects_custom_blocks() {
        // GIVEN
        let default_config = DisplayConfig::default();
        let config = DisplayConfig {
            pending_block: String::from("-"),
            complete_block: String::from("+"),
            ..default_config
        };

        // WHEN
        let got = get_progress_bar(18 * 60, config);

        // THEN
        let expected = String::from(" +++++++--- ");

        assert_eq!(got, expected);
    }

    #[test]
    fn get_progress_respects_num_blocks() {
        // GIVEN
        let default_config = DisplayConfig::default();
        let config = DisplayConfig {
            num_blocks: 5,
            ..default_config
        };

        // WHEN
        let got = get_progress_bar(10 * 60, config);

        // THEN
        let expected = String::from(" ▪▪▫▫▫ ");

        assert_eq!(got, expected);
    }

    #[test]
    fn get_progress_respects_custom_delimiter() {
        // GIVEN
        let default_config = DisplayConfig::default();
        let config = DisplayConfig {
            delimiter: String::from(" "),
            ..default_config
        };

        // WHEN
        let got = get_progress_bar(10 * 60, config);

        // THEN
        let expected = String::from(" ▪ ▪ ▪ ▪ ▫ ▫ ▫ ▫ ▫ ▫ ");

        assert_eq!(got, expected);
    }
}
