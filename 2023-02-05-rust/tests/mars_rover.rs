use kata_mars_rover::{run, Direction};

#[test]
fn test_runs_unparseable_program() {
    let result = run(0, 0, Direction::East, "???");
    assert!(result.is_err());
}

#[test]
fn test_runs_noop_program() {
    let result = run(0, 0, Direction::East, " ");
    assert_eq!(result, Ok("(0, 0) E".to_string()));
}

#[test]
fn test_runs_rover_moving_forward() {
    let result = run(0, 0, Direction::East, "ff");
    assert_eq!(result, Ok("(2, 0) E".to_string()));
}
