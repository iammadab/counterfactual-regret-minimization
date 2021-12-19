pub struct RPS {}

impl RPS {
    pub const NO_OF_ACTIONS: usize = 3;

    fn value(action1: usize, action2: usize) -> f64 {
        if action1 == action2 {
            return 0.0;
        }
        if ((action1 + 1) % 3) == action2 {
            return -1.0;
        }
        return 1.0;
    }

    pub fn action_utilities(opponent_action: usize) -> Vec<f64> {
        let mut action_utilities = vec![0.0; 3];
        for i in 0..3 {
            action_utilities[i] = RPS::value(i, opponent_action);
        }
        action_utilities
    }
}

#[test]
fn test_value_function() {
    const ROCK: usize = 0;
    const PAPER: usize = 1;
    const SCISSORS: usize = 2;

    // Exhaust Rock
    assert_eq!(RPS::value(ROCK, PAPER), -1.0);
    assert_eq!(RPS::value(ROCK, SCISSORS), 1.0);
    assert_eq!(RPS::value(ROCK, ROCK), 0.0);

    // Exhaust Paper
    assert_eq!(RPS::value(PAPER, PAPER), 0.0);
    assert_eq!(RPS::value(PAPER, SCISSORS), -1.0);
    assert_eq!(RPS::value(PAPER, ROCK), 1.0);

    // Exhaust Scissors
    assert_eq!(RPS::value(SCISSORS, PAPER), 1.0);
    assert_eq!(RPS::value(SCISSORS, SCISSORS), 0.0);
    assert_eq!(RPS::value(SCISSORS, ROCK), -1.0);
}
