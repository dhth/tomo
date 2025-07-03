mod common;

use common::Fixture;
use insta_cmd::assert_cmd_snapshot;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn starting_timer_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut start_cmd = fx.cmd(["start"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    let mut show_cmd = fx.base_cmd();
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
    let fx = Fixture::new();
    let mut start_cmd = fx.cmd(["start", "--elapsed-mins", "8"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(start_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    let mut show_cmd = fx.base_cmd();
    assert_cmd_snapshot!(show_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
     ▪▪▪▫▫▫▫▫▫▫ 

    ----- stderr -----
    ");
}

//------------//
//  FAILURES  //
//------------//

#[test]
fn start_fails_if_elapsed_mins_is_greater_than_threshold() {
    // GIVEN
    let fx = Fixture::new();
    let mut start_cmd = fx.cmd(["start", "--elapsed-mins", "21"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(start_cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: elapsed mins cannot be greater than 20
    ");
}
