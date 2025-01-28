use assert_cmd::Command;
use pretty_assertions::assert_eq;
use tempdir::TempDir;

struct TestFixture {
    _temp_dir: TempDir,
    data_file_path: String,
}

impl TestFixture {
    fn new() -> Self {
        let temp_dir =
            TempDir::new("tomo-integration-tests").expect("couldn't create temporary directory");
        let data_file_path = temp_dir
            .path()
            .join(".tomo")
            .to_str()
            .expect("temporary directory path is not valid utf-8")
            .to_string();
        Self {
            _temp_dir: temp_dir,
            data_file_path,
        }
    }
}

#[allow(dead_code)]
trait ExpectedSuccess {
    fn print_stderr_if_failed(&self, context: &str);
}

impl ExpectedSuccess for std::process::Output {
    fn print_stderr_if_failed(&self, context: &str) {
        if self.status.success() {
            return;
        }

        let stderr = std::str::from_utf8(&self.stderr).expect("invalid utf-8 stderr");
        println!("{} stderr: \n{}", context, stderr);
    }
}

#[allow(dead_code)]
trait ExpectedFailure {
    fn print_stdout_if_succeeded(&self, context: &str);
}

impl ExpectedFailure for std::process::Output {
    fn print_stdout_if_succeeded(&self, context: &str) {
        if !self.status.success() {
            return;
        }

        let stdout = std::str::from_utf8(&self.stdout).expect("invalid utf-8 stdout");
        println!("{} stdout: \n{}", context, stdout);
    }
}

// SUCCESSES
#[test]
fn shows_help() {
    // GIVEN
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--help");
    let output = cmd.output().expect("running command failed");

    // THEN
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    assert!(stdout.contains("tomo is a no-frills pomodoro progress indicator for tmux"));
}

#[test]
fn starting_timer_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let data_file_flag = format!("--data-file={}", fixture.data_file_path.as_str());

    // WHEN
    let mut start_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    start_cmd.arg("start");
    start_cmd.arg(&data_file_flag);
    let start_output = start_cmd.output().expect("running command failed");

    let mut show_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    show_cmd.arg(&data_file_flag);
    let show_output = show_cmd.output().expect("running command failed");

    // THEN
    start_output.print_stderr_if_failed("start");
    assert!(start_output.status.success());

    show_output.print_stderr_if_failed("show");
    assert!(show_output.status.success());
    let show_stdout = String::from_utf8(show_output.stdout).expect("invalid utf-8 stdout");
    assert_eq!(show_stdout, " ▫▫▫▫▫▫▫▫▫▫ \n");
}

#[test]
fn starting_timer_with_elapsed_time_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let data_file_flag = format!("--data-file={}", fixture.data_file_path.as_str());

    // WHEN
    let mut start_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    start_cmd.arg("start");
    start_cmd.arg("-e=8");
    start_cmd.arg(&data_file_flag);
    let start_output = start_cmd.output().expect("running command failed");

    let mut show_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    show_cmd.arg(&data_file_flag);
    let show_output = show_cmd.output().expect("running command failed");

    // THEN
    start_output.print_stderr_if_failed("start");
    assert!(start_output.status.success());

    show_output.print_stderr_if_failed("show");
    assert!(show_output.status.success());
    let show_stdout = String::from_utf8(show_output.stdout).expect("invalid utf-8 stdout");
    assert_eq!(show_stdout, " ▪▪▪▫▫▫▫▫▫▫ \n");
}

#[test]
fn using_custom_delimiter_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let data_file_flag = format!("--data-file={}", fixture.data_file_path.as_str());

    // WHEN
    let mut start_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    start_cmd.arg("start");
    start_cmd.arg(&data_file_flag);
    let start_output = start_cmd.output().expect("running command failed");

    let mut show_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    show_cmd.arg("-d= ");
    show_cmd.arg(&data_file_flag);
    let show_output = show_cmd.output().expect("running command failed");

    // THEN
    start_output.print_stderr_if_failed("start");
    assert!(start_output.status.success());

    show_output.print_stderr_if_failed("show");
    assert!(show_output.status.success());
    let show_stdout = String::from_utf8(show_output.stdout).expect("invalid utf-8 stdout");
    assert_eq!(show_stdout, " ▫ ▫ ▫ ▫ ▫ ▫ ▫ ▫ ▫ ▫ \n");
}

#[test]
fn using_custom_num_blocks_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let data_file_flag = format!("--data-file={}", fixture.data_file_path.as_str());

    // WHEN
    let mut start_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    start_cmd.arg("start");
    start_cmd.arg("-e=12");
    start_cmd.arg(&data_file_flag);
    let start_output = start_cmd.output().expect("running command failed");

    let mut show_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    show_cmd.arg(&data_file_flag);
    show_cmd.arg("-n=5");
    let show_output = show_cmd.output().expect("running command failed");

    // THEN
    start_output.print_stderr_if_failed("start");
    assert!(start_output.status.success());

    show_output.print_stderr_if_failed("show");
    assert!(show_output.status.success());
    let show_stdout = String::from_utf8(show_output.stdout).expect("invalid utf-8 stdout");
    assert_eq!(show_stdout, " ▪▪▫▫▫ \n");
}

#[test]
fn using_custom_pending_block_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let data_file_flag = format!("--data-file={}", fixture.data_file_path.as_str());

    // WHEN
    let mut start_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    start_cmd.arg("start");
    start_cmd.arg("-e=12");
    start_cmd.arg(&data_file_flag);
    let start_output = start_cmd.output().expect("running command failed");

    let mut show_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    show_cmd.arg(&data_file_flag);
    show_cmd.arg("-p=.");
    let show_output = show_cmd.output().expect("running command failed");

    // THEN
    start_output.print_stderr_if_failed("start");
    assert!(start_output.status.success());

    show_output.print_stderr_if_failed("show");
    assert!(show_output.status.success());
    let show_stdout = String::from_utf8(show_output.stdout).expect("invalid utf-8 stdout");
    assert_eq!(show_stdout, " ▪▪▪▪...... \n");
}

#[test]
fn using_custom_complete_block_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let data_file_flag = format!("--data-file={}", fixture.data_file_path.as_str());

    // WHEN
    let mut start_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    start_cmd.arg("start");
    start_cmd.arg("-e=12");
    start_cmd.arg(&data_file_flag);
    let start_output = start_cmd.output().expect("running command failed");

    let mut show_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    show_cmd.arg(&data_file_flag);
    show_cmd.arg("-c=x");
    let show_output = show_cmd.output().expect("running command failed");

    // THEN
    start_output.print_stderr_if_failed("start");
    assert!(start_output.status.success());

    show_output.print_stderr_if_failed("show");
    assert!(show_output.status.success());
    let show_stdout = String::from_utf8(show_output.stdout).expect("invalid utf-8 stdout");
    assert_eq!(show_stdout, " xxxx▫▫▫▫▫▫ \n");
}

#[test]
fn using_custom_left_pad_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let data_file_flag = format!("--data-file={}", fixture.data_file_path.as_str());

    // WHEN
    let mut start_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    start_cmd.arg("start");
    start_cmd.arg("-e=12");
    start_cmd.arg(&data_file_flag);
    let start_output = start_cmd.output().expect("running command failed");

    let mut show_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    show_cmd.arg(&data_file_flag);
    show_cmd.arg("-l=[");
    let show_output = show_cmd.output().expect("running command failed");

    // THEN
    start_output.print_stderr_if_failed("start");
    assert!(start_output.status.success());

    show_output.print_stderr_if_failed("show");
    assert!(show_output.status.success());
    let show_stdout = String::from_utf8(show_output.stdout).expect("invalid utf-8 stdout");
    assert_eq!(show_stdout, "[▪▪▪▪▫▫▫▫▫▫ \n");
}

#[test]
fn using_custom_right_pad_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let data_file_flag = format!("--data-file={}", fixture.data_file_path.as_str());

    // WHEN
    let mut start_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    start_cmd.arg("start");
    start_cmd.arg("-e=12");
    start_cmd.arg(&data_file_flag);
    let start_output = start_cmd.output().expect("running command failed");

    let mut show_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    show_cmd.arg(&data_file_flag);
    show_cmd.arg("-r=]");
    let show_output = show_cmd.output().expect("running command failed");

    // THEN
    start_output.print_stderr_if_failed("start");
    assert!(start_output.status.success());

    show_output.print_stderr_if_failed("show");
    assert!(show_output.status.success());
    let show_stdout = String::from_utf8(show_output.stdout).expect("invalid utf-8 stdout");
    assert_eq!(show_stdout, " ▪▪▪▪▫▫▫▫▫▫]\n");
}

#[test]
fn starting_a_break_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let data_file_flag = format!("--data-file={}", fixture.data_file_path.as_str());

    // WHEN
    let mut break_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    break_cmd.arg("break");
    break_cmd.arg(&data_file_flag);
    let start_output = break_cmd.output().expect("running command failed");

    let mut show_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    show_cmd.arg(&data_file_flag);
    let show_output = show_cmd.output().expect("running command failed");

    // THEN
    start_output.print_stderr_if_failed("start");
    assert!(start_output.status.success());

    show_output.print_stderr_if_failed("show");
    assert!(show_output.status.success());
    let show_stdout = String::from_utf8(show_output.stdout).expect("invalid utf-8 stdout");
    assert_eq!(show_stdout, " \\o/ ");
}

#[test]
fn using_a_custom_break_message_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let data_file_flag = format!("--data-file={}", fixture.data_file_path.as_str());

    // WHEN
    let mut break_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    break_cmd.arg("break");
    break_cmd.arg(&data_file_flag);
    let start_output = break_cmd.output().expect("running command failed");

    let mut show_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    show_cmd.arg(&data_file_flag);
    show_cmd.arg("--break-msg=done!");
    let show_output = show_cmd.output().expect("running command failed");

    // THEN
    start_output.print_stderr_if_failed("start");
    assert!(start_output.status.success());

    show_output.print_stderr_if_failed("show");
    assert!(show_output.status.success());
    let show_stdout = String::from_utf8(show_output.stdout).expect("invalid utf-8 stdout");
    assert_eq!(show_stdout, " done! ");
}

#[test]
fn stopping_a_timer_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let data_file_flag = format!("--data-file={}", fixture.data_file_path.as_str());

    // WHEN
    let mut start_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    start_cmd.arg("start");
    start_cmd.arg("-e=12");
    start_cmd.arg(&data_file_flag);
    let start_output = start_cmd.output().expect("running start command failed");

    let mut stop_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    stop_cmd.arg("break");
    stop_cmd.arg(&data_file_flag);
    let stop_output = stop_cmd.output().expect("running stop command failed");

    let mut show_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    show_cmd.arg(&data_file_flag);
    let show_output = show_cmd.output().expect("running command failed");

    // THEN
    start_output.print_stderr_if_failed("start");
    assert!(start_output.status.success());

    if !stop_output.status.success() {
        let stderr = String::from_utf8(stop_output.stderr).expect("invalid utf-8 stderr");
        println!("stop stderr: \n{}", stderr);
    }
    assert!(stop_output.status.success());

    show_output.print_stderr_if_failed("show");
    assert!(show_output.status.success());
    let show_stdout = String::from_utf8(show_output.stdout).expect("invalid utf-8 stdout");
    assert_eq!(show_stdout, " \\o/ ");
}

#[test]
fn using_multiple_flags_together_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let data_file_flag = format!("--data-file={}", fixture.data_file_path.as_str());

    // WHEN
    let mut start_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    start_cmd.arg("start");
    start_cmd.arg("-e=12");
    start_cmd.arg(&data_file_flag);
    let start_output = start_cmd.output().expect("running command failed");

    let mut show_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    show_cmd.arg(&data_file_flag);
    show_cmd.arg("-p=o");
    show_cmd.arg("-c=x");
    show_cmd.arg("-d=|");
    show_cmd.arg("-l=[[ ");
    show_cmd.arg("-r= ]]");
    show_cmd.arg("-n=5");
    let show_output = show_cmd.output().expect("running command failed");

    // THEN
    start_output.print_stderr_if_failed("start");
    assert!(start_output.status.success());

    show_output.print_stderr_if_failed("show");
    assert!(show_output.status.success());
    let show_stdout = String::from_utf8(show_output.stdout).expect("invalid utf-8 stdout");
    assert_eq!(show_stdout, "[[ x|x|o|o|o ]]\n");
}

// FAILURES
#[test]
fn fails_if_num_blocks_is_greater_than_threshold() {
    // GIVEN
    let fixture = TestFixture::new();
    let data_file_flag = format!("--data-file={}", fixture.data_file_path.as_str());

    // WHEN
    let mut start_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    start_cmd.arg("start");
    start_cmd.arg(&data_file_flag);
    let start_output = start_cmd.output().expect("running command failed");

    let mut show_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    show_cmd.arg(&data_file_flag);
    show_cmd.arg("-n=101");
    let show_output = show_cmd.output().expect("running command failed");

    // THEN
    start_output.print_stderr_if_failed("start");
    assert!(start_output.status.success());

    show_output.print_stdout_if_succeeded("show");
    assert!(!show_output.status.success());
    let stderr = String::from_utf8(show_output.stderr).expect("invalid utf-8 stderr");
    assert!(stderr.contains("Error: number of blocks needs to be between 3 and 100"));
}

#[test]
fn fails_if_num_blocks_is_less_than_threshold() {
    // GIVEN
    let fixture = TestFixture::new();
    let data_file_flag = format!("--data-file={}", fixture.data_file_path.as_str());

    // WHEN
    let mut start_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    start_cmd.arg("start");
    start_cmd.arg(&data_file_flag);
    let start_output = start_cmd.output().expect("running command failed");

    let mut show_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    show_cmd.arg(&data_file_flag);
    show_cmd.arg("-n=2");
    let show_output = show_cmd.output().expect("running command failed");

    // THEN
    start_output.print_stderr_if_failed("start");
    assert!(start_output.status.success());

    show_output.print_stdout_if_succeeded("show");
    assert!(!show_output.status.success());
    let stderr = String::from_utf8(show_output.stderr).expect("invalid utf-8 stderr");
    assert!(stderr.contains("number of blocks needs to be between 3 and 100"));
}

#[test]
fn start_fails_if_elapsed_mins_is_greater_than_threshold() {
    // GIVEN
    let fixture = TestFixture::new();
    let data_file_flag = format!("--data-file={}", fixture.data_file_path.as_str());

    // WHEN
    let mut start_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    start_cmd.arg("start");
    start_cmd.arg("-e=21");
    start_cmd.arg(&data_file_flag);
    let start_output = start_cmd.output().expect("running command failed");

    // THEN
    start_output.print_stdout_if_succeeded("show");
    assert!(!start_output.status.success());
    let stderr = String::from_utf8(start_output.stderr).expect("invalid utf-8 stderr");
    assert!(stderr.contains("elapsed mins cannot be greater than 20"));
}
