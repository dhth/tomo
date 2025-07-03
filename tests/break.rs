mod common;

use common::Fixture;
use insta_cmd::assert_cmd_snapshot;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn starting_a_break_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut break_cmd = fx.cmd(["break"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(break_cmd, @r"
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
     \o/ 
    ----- stderr -----
    ");
}
