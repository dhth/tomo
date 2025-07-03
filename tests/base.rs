mod common;

use common::Fixture;
use insta_cmd::assert_cmd_snapshot;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn shows_help() {
    // GIVEN
    // WHEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["--help"]);

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
fn using_custom_delimiter_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut start_cmd = fx.cmd(["start"]);
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    // WHEN
    // THEN
    let mut show_cmd = fx.cmd(["--delimiter", " "]);
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
    let fx = Fixture::new();
    let mut start_cmd = fx.cmd(["start", "--elapsed-mins", "12"]);
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    // WHEN
    // THEN
    let mut show_cmd = fx.cmd(["--num-blocks", "5"]);
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
    let fx = Fixture::new();
    let mut start_cmd = fx.cmd(["start", "--elapsed-mins", "12"]);
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    // WHEN
    // THEN
    let mut show_cmd = fx.cmd(["--pending-block", "."]);
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
    let fx = Fixture::new();
    let mut start_cmd = fx.cmd(["start", "--elapsed-mins", "12"]);
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    // WHEN
    // THEN
    let mut show_cmd = fx.cmd(["--complete-block", "x"]);
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
    let fx = Fixture::new();
    let mut start_cmd = fx.cmd(["start", "--elapsed-mins", "12"]);
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    // WHEN
    // THEN
    let mut show_cmd = fx.cmd(["--left-pad", "["]);
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
    let fx = Fixture::new();
    let mut start_cmd = fx.cmd(["start", "--elapsed-mins", "12"]);
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    // WHEN
    // THEN
    let mut show_cmd = fx.cmd(["--right-pad", "]"]);
    assert_cmd_snapshot!(show_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
     ▪▪▪▪▫▫▫▫▫▫]

    ----- stderr -----
    ");
}

#[test]
fn using_multiple_flags_together_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut start_cmd = fx.cmd(["start", "--elapsed-mins", "12"]);
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    // WHEN
    // THEN
    let mut show_cmd = fx.cmd([
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
    ]);
    assert_cmd_snapshot!(show_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    [[ x|x|o|o|o ]]

    ----- stderr -----
    ");
}

#[test]
fn using_a_custom_break_message_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut break_cmd = fx.cmd(["break"]);
    assert_cmd_snapshot!(break_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    // WHEN
    // THEN
    let mut show_cmd = fx.cmd(["--break-msg", "done!"]);
    assert_cmd_snapshot!(show_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
     done! 
    ----- stderr -----
    ");
}

//------------//
//  FAILURES  //
//------------//

#[test]
fn fails_if_num_blocks_is_greater_than_threshold() {
    // GIVEN
    let fx = Fixture::new();
    let mut start_cmd = fx.cmd(["start"]);
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    // WHEN
    // THEN
    let mut show_cmd = fx.cmd(["--num-blocks", "101"]);
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
    let fx = Fixture::new();
    let mut start_cmd = fx.cmd(["start"]);
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    // WHEN
    // THEN
    let mut show_cmd = fx.cmd(["--num-blocks", "2"]);
    assert_cmd_snapshot!(show_cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: number of blocks needs to be between 3 and 100
    ");
}
