use kata_mars_rover::{run, Direction};

#[test]
fn test_runs_unparseable_program() {
    let result = run(0, 0, Direction::East, "???", None);
    assert!(result.is_err());
}

#[test]
fn test_runs_noop_program() {
    let result = run(0, 0, Direction::East, " ", None);
    assert_eq!(result, Ok("(0, 0) E".to_string()));
}

#[test]
fn test_runs_rover_moving_and_turning() {
    let result = run(0, 0, Direction::East, "ffblrr", None);
    assert_eq!(result, Ok("(1, 0) S".to_string()));
}

#[test]
fn test_runs_rover_halting_at_obstacles() {
    let map_path = std::path::PathBuf::from("tests/fixtures/map_with_obstacle_at_2_0.txt");
    let result = run(0, 0, Direction::East, "fffff", Some(map_path));

    assert_eq!(result, Err("(1, 0) E".to_string()));
}
