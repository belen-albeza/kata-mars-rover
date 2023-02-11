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
fn test_runs_rover_moving_and_turning() {
    let result = run(0, 0, Direction::East, "ffblrr");
    assert_eq!(result, Ok("(1, 0) S".to_string()));
}
