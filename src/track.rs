use crate::config::DisplayConfig;
use anyhow::Context;
use chrono::DateTime;
use chrono::prelude::*;
use std::fs::{self};
use std::path::PathBuf;

const BREAK_STRING: &str = "break";
const STOP_STRING: &str = "stop";

pub fn start_tracking(file_path: &PathBuf, time: DateTime<Utc>) -> anyhow::Result<()> {
    fs::write(file_path, time.to_rfc3339()).context("couldn't write to tomo's data file")
}

pub fn take_break(file_path: &PathBuf) -> anyhow::Result<()> {
    fs::write(file_path, BREAK_STRING).context("couldn't write to tomo's data file")
}

pub fn stop_tracking(file_path: &PathBuf) -> Result<(), anyhow::Error> {
    fs::write(file_path, STOP_STRING).context("couldn't write to tomo's data file")
}

pub fn show_progress(
    file_path: &PathBuf,
    now: DateTime<Utc>,
    config: &DisplayConfig,
) -> anyhow::Result<()> {
    let status = fs::read_to_string(file_path).context("couldn't read from tomo's data file")?;

    if status == STOP_STRING {
        return Ok(());
    }

    if status == BREAK_STRING {
        print!(
            "{}{}{}",
            config.left_pad, config.break_msg, config.right_pad,
        );
        return Ok(());
    }

    let ts_trimmed = status.trim();

    let ts = DateTime::parse_from_rfc3339(ts_trimmed)
        .context("couldn't not parse time from tomo's data file")?;

    let diff_seconds = now.signed_duration_since(ts.to_utc()).num_seconds();
    let output = get_progress_bar(diff_seconds, config);
    println!("{output}");

    Ok(())
}

pub fn get_progress_bar(diff_seconds: i64, config: &DisplayConfig) -> String {
    let chunks = diff_seconds / (25 * 60 / (config.num_blocks as i64));

    if chunks >= config.num_blocks as i64 {
        return format!(
            "{}{}{}",
            config.left_pad, config.finished_msg, config.right_pad,
        );
    }

    let mut bar = String::new();

    for _ in 0..chunks {
        bar.push_str(&config.complete_block);
        if !config.delimiter.is_empty() {
            bar.push_str(&config.delimiter);
        }
    }

    for _ in 0..((config.num_blocks as i64) - chunks - 1) {
        bar.push_str(&config.pending_block);
        if !config.delimiter.is_empty() {
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
        let got = get_progress_bar(10 * 60, &config);

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
        let got = get_progress_bar(0, &config);

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
        let got = get_progress_bar(18 * 60, &config);

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
        let got = get_progress_bar(10 * 60, &config);

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
        let got = get_progress_bar(10 * 60, &config);

        // THEN
        let expected = String::from(" ▪ ▪ ▪ ▪ ▫ ▫ ▫ ▫ ▫ ▫ ");

        assert_eq!(got, expected);
    }

    #[test]
    fn get_progress_shows_empty_progress_bar() {
        // GIVEN
        let config = DisplayConfig::default();

        // WHEN
        let got_at_start = get_progress_bar(0, &config);
        let got_at_min_one = get_progress_bar(60, &config);

        // THEN
        let expected = String::from(" ▫▫▫▫▫▫▫▫▫▫ ");

        assert_eq!(got_at_start, expected);
        assert_eq!(got_at_min_one, expected);
    }

    #[test]
    fn get_progress_shows_finished_msg_when_timer_is_finished() {
        // GIVEN
        let default_config = DisplayConfig::default();
        let config = DisplayConfig {
            left_pad: String::from("[["),
            right_pad: String::from("]]"),
            finished_msg: String::from("fertig"),
            ..default_config
        };

        // WHEN
        let got_at_min_25 = get_progress_bar(25 * 60, &config);
        let got_at_min_26 = get_progress_bar(26 * 60, &config);

        // THEN
        let expected = String::from("[[fertig]]");

        assert_eq!(got_at_min_25, expected);
        assert_eq!(got_at_min_26, expected);
    }
}
