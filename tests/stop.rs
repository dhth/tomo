mod common;

use common::Fixture;
use insta_cmd::assert_cmd_snapshot;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn stopping_a_timer_works() {
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
    let mut stop_cmd = fx.cmd(["stop"]);
    assert_cmd_snapshot!(stop_cmd, @r"
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

    ----- stderr -----
    ");
}
