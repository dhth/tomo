use tempfile::TempDir;

use std::process::Command;

use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};

fn base_command() -> Command {
    Command::new(get_cargo_bin("tomo"))
}

struct TestFixture {
    _temp_dir: TempDir,
    data_file_path: String,
}

impl TestFixture {
    fn new() -> Self {
        let temp_dir = TempDir::new().expect("couldn't create temporary directory");
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

// SUCCESSES
#[test]
fn shows_help() {
    // GIVEN
    // WHEN
    let mut base_cmd = base_command();
    let mut cmd = base_cmd.args(["--help"]);

    // THEN
    assert_cmd_snapshot!(cmd, @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    tomo is a no-frills pomodoro progress indicator for tmux

    Usage: tomo [OPTIONS] [COMMAND]

    Commands:
      start  Start a pomodoro timer
      stop   Stop timer
      break  Start a break
      help   Print this message or the help of the given subcommand(s)

    Options:
      -p, --pending-block <STRING>   String to represent a "pending" block in the progress bar [default: ▫]
      -c, --complete-block <STRING>  String to represent a "complete" block in the progress bar [default: ▪]
      -l, --left-pad <STRING>        String to pad the output with on the LHS [default: " "]
      -r, --right-pad <STRING>       String to pad the output with on the RHS [default: " "]
      -d, --delimiter <STRING>       Delimiter between progress bar chunks [default: ]
      -n, --num-blocks <NUM>         Number of blocks to show in progress bar [default: 10]
          --finished-msg <STRING>    Message to show when timer is finished [default: done]
          --break-msg <STRING>       Message to show when on a break [default: \o/]
          --data-file <STRING>       tomo's data file (defaults to <YOUR_DATA_DIR>/tomo/.tomo)
      -h, --help                     Print help

    ----- stderr -----
    "#);
}

#[test]
fn starting_timer_works() {
    // GIVEN
    let fixture = TestFixture::new();

    // WHEN
    // THEN
    let mut base_cmd = base_command();
    let mut start_cmd = base_cmd.args(["--data-file", fixture.data_file_path.as_str(), "start"]);
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    let mut base_show_cmd = base_command();
    let mut show_cmd = base_show_cmd.args(["--data-file", fixture.data_file_path.as_str()]);
    assert_cmd_snapshot!(show_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
     ▫▫▫▫▫▫▫▫▫▫ 

    ----- stderr -----
    ");
}

#[test]
fn starting_timer_with_elapsed_time_works() {
    // GIVEN
    let fixture = TestFixture::new();

    // WHEN
    // THEN
    let mut base_cmd = base_command();
    let mut start_cmd = base_cmd.args([
        "start",
        "--elapsed-mins",
        "8",
        "--data-file",
        fixture.data_file_path.as_str(),
    ]);
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    let mut base_show_cmd = base_command();
    let mut show_cmd = base_show_cmd.args(["--data-file", fixture.data_file_path.as_str()]);
    assert_cmd_snapshot!(show_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
     ▪▪▪▫▫▫▫▫▫▫ 

    ----- stderr -----
    ");
}

#[test]
fn using_custom_delimiter_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let mut base_cmd = base_command();

    // WHEN
    // THEN
    let mut start_cmd = base_cmd.args(["start", "--data-file", fixture.data_file_path.as_str()]);
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    let mut base_show_cmd = base_command();
    let mut show_cmd = base_show_cmd.args(["--delimiter", " ", "--data-file", fixture.data_file_path.as_str()]);
    assert_cmd_snapshot!(show_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
     ▫ ▫ ▫ ▫ ▫ ▫ ▫ ▫ ▫ ▫ 

    ----- stderr -----
    ");
}

#[test]
fn using_custom_num_blocks_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let mut base_cmd = base_command();

    // WHEN
    // THEN
    let mut start_cmd = base_cmd.args([
        "start",
        "--elapsed-mins",
        "12",
        "--data-file",
        fixture.data_file_path.as_str(),
    ]);
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    let mut base_show_cmd = base_command();
    let mut show_cmd = base_show_cmd.args(["--num-blocks", "5", "--data-file", fixture.data_file_path.as_str()]);
    assert_cmd_snapshot!(show_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
     ▪▪▫▫▫ 

    ----- stderr -----
    ");
}

#[test]
fn using_custom_pending_block_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let mut base_cmd = base_command();

    // WHEN
    // THEN
    let mut start_cmd = base_cmd.args([
        "start",
        "--elapsed-mins",
        "12",
        "--data-file",
        fixture.data_file_path.as_str(),
    ]);
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    let mut base_show_cmd = base_command();
    let mut show_cmd = base_show_cmd.args(["--pending-block", ".", "--data-file", fixture.data_file_path.as_str()]);
    assert_cmd_snapshot!(show_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
     ▪▪▪▪...... 

    ----- stderr -----
    ");
}

#[test]
fn using_custom_complete_block_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let mut base_cmd = base_command();

    // WHEN
    // THEN
    let mut start_cmd = base_cmd.args([
        "start",
        "--elapsed-mins",
        "12",
        "--data-file",
        fixture.data_file_path.as_str(),
    ]);
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    let mut base_show_cmd = base_command();
    let mut show_cmd = base_show_cmd.args(["--complete-block", "x", "--data-file", fixture.data_file_path.as_str()]);
    assert_cmd_snapshot!(show_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
     xxxx▫▫▫▫▫▫ 

    ----- stderr -----
    ");
}

#[test]
fn using_custom_left_pad_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let mut base_cmd = base_command();

    // WHEN
    // THEN
    let mut start_cmd = base_cmd.args([
        "start",
        "--elapsed-mins",
        "12",
        "--data-file",
        fixture.data_file_path.as_str(),
    ]);
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    let mut base_show_cmd = base_command();
    let mut show_cmd = base_show_cmd.args(["--left-pad", "[", "--data-file", fixture.data_file_path.as_str()]);
    assert_cmd_snapshot!(show_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    [▪▪▪▪▫▫▫▫▫▫ 

    ----- stderr -----
    ");
}

#[test]
fn using_custom_right_pad_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let mut base_cmd = base_command();

    // WHEN
    // THEN
    let mut start_cmd = base_cmd.args([
        "start",
        "--elapsed-mins",
        "12",
        "--data-file",
        fixture.data_file_path.as_str(),
    ]);
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    let mut base_show_cmd = base_command();
    let mut show_cmd = base_show_cmd.args(["--right-pad", "]", "--data-file", fixture.data_file_path.as_str()]);
    assert_cmd_snapshot!(show_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
     ▪▪▪▪▫▫▫▫▫▫]

    ----- stderr -----
    ");
}

#[test]
fn starting_a_break_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let mut base_cmd = base_command();

    // WHEN
    // THEN
    let mut break_cmd = base_cmd.args(["break", "--data-file", fixture.data_file_path.as_str()]);
    assert_cmd_snapshot!(break_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    let mut base_show_cmd = base_command();
    let mut show_cmd = base_show_cmd.args(["--data-file", fixture.data_file_path.as_str()]);
    assert_cmd_snapshot!(show_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
     \o/ 
    ----- stderr -----
    ");
}

#[test]
fn using_a_custom_break_message_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let mut base_cmd = base_command();

    // WHEN
    // THEN
    let mut break_cmd = base_cmd.args(["break", "--data-file", fixture.data_file_path.as_str()]);
    assert_cmd_snapshot!(break_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    let mut base_show_cmd = base_command();
    let mut show_cmd = base_show_cmd.args([
        "--break-msg",
        "done!",
        "--data-file",
        fixture.data_file_path.as_str(),
    ]);
    assert_cmd_snapshot!(show_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
     done! 
    ----- stderr -----
    ");
}

#[test]
fn stopping_a_timer_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let mut base_cmd = base_command();

    // WHEN
    // THEN
    let mut start_cmd = base_cmd.args([
        "start",
        "--elapsed-mins",
        "12",
        "--data-file",
        fixture.data_file_path.as_str(),
    ]);
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    let mut base_stop_cmd = base_command();
    let mut stop_cmd =
        base_stop_cmd.args(["break", "--data-file", fixture.data_file_path.as_str()]);
    assert_cmd_snapshot!(stop_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    let mut base_show_cmd = base_command();
    let mut show_cmd = base_show_cmd.args(["--data-file", fixture.data_file_path.as_str()]);
    assert_cmd_snapshot!(show_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
     \o/ 
    ----- stderr -----
    ");
}

#[test]
fn using_multiple_flags_together_works() {
    // GIVEN
    let fixture = TestFixture::new();
    let mut base_cmd = base_command();

    // WHEN
    // THEN
    let mut start_cmd = base_cmd.args([
        "start",
        "--elapsed-mins",
        "12",
        "--data-file",
        fixture.data_file_path.as_str(),
    ]);
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    let mut base_show_cmd = base_command();
    let mut show_cmd = base_show_cmd.args([
        "--pending-block",
        "o",
        "--complete-block",
        "x",
        "--delimiter",
        "|",
        "--left-pad",
        "[[ ",
        "--right-pad",
        " ]]",
        "--num-blocks",
        "5",
        "--data-file",
        fixture.data_file_path.as_str(),
    ]);
    assert_cmd_snapshot!(show_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    [[ x|x|o|o|o ]]

    ----- stderr -----
    ");
}

// FAILURES
#[test]
fn fails_if_num_blocks_is_greater_than_threshold() {
    // GIVEN
    let fixture = TestFixture::new();
    let mut base_cmd = base_command();

    // WHEN
    // THEN
    let mut start_cmd = base_cmd.args(["start", "--data-file", fixture.data_file_path.as_str()]);
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    let mut base_show_cmd = base_command();
    let mut show_cmd =
        base_show_cmd.args(["--num-blocks", "101", "--data-file", fixture.data_file_path.as_str()]);
    assert_cmd_snapshot!(show_cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: number of blocks needs to be between 3 and 100
    ");
}

#[test]
fn fails_if_num_blocks_is_less_than_threshold() {
    // GIVEN
    let fixture = TestFixture::new();
    let mut base_cmd = base_command();

    // WHEN
    // THEN
    let mut start_cmd = base_cmd.args(["start", "--data-file", fixture.data_file_path.as_str()]);
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    let mut base_show_cmd = base_command();
    let mut show_cmd = base_show_cmd.args(["--num-blocks", "2", "--data-file", fixture.data_file_path.as_str()]);
    assert_cmd_snapshot!(show_cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: number of blocks needs to be between 3 and 100
    ");
}

#[test]
fn start_fails_if_elapsed_mins_is_greater_than_threshold() {
    // GIVEN
    let fixture = TestFixture::new();
    let mut base_cmd = base_command();

    // WHEN
    // THEN
    let mut start_cmd = base_cmd.args([
        "start",
        "--elapsed-mins",
        "21",
        "--data-file",
        fixture.data_file_path.as_str(),
    ]);
    assert_cmd_snapshot!(start_cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: elapsed mins cannot be greater than 20
    ");
}
