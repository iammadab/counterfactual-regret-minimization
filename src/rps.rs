pub trait Game {
    type Action;
    fn no_of_actions(&self) -> usize;
    fn value(&self, action_1: Self::Action, action_2: Self::Action) -> f64;
    fn action_utilities(&self, opponent_action: usize) -> Vec<f64>;
}

pub struct RPS {}

impl Game for RPS {
    type Action = usize;

    fn no_of_actions(&self) -> usize {
        3
    }

    fn value(&self, action1: Self::Action, action2: Self::Action) -> f64 {
        if action1 == action2 {
            return 0.0;
        }
        if ((action1 + 1) % 3) == action2 {
            return -1.0;
        }
        return 1.0;
    }

    fn action_utilities(&self, opponent_action: usize) -> Vec<f64> {
        let mut action_utilities = vec![0.0; 3];
        for i in 0..3 {
            action_utilities[i] = self.value(i, opponent_action);
        }
        action_utilities
    }
}

#[test]
fn test_value_function() {
    const ROCK: usize = 0;
    const PAPER: usize = 1;
    const SCISSORS: usize = 2;

    let rps_environment = RPS {};

    // Exhaust Rock
    assert_eq!(rps_environment.value(ROCK, PAPER), -1.0);
    assert_eq!(rps_environment.value(ROCK, SCISSORS), 1.0);
    assert_eq!(rps_environment.value(ROCK, ROCK), 0.0);

    // Exhaust Paper
    assert_eq!(rps_environment.value(PAPER, PAPER), 0.0);
    assert_eq!(rps_environment.value(PAPER, SCISSORS), -1.0);
    assert_eq!(rps_enviroment.value(PAPER, ROCK), 1.0);

    // Exhaust Scissors
    assert_eq!(rps_enviroment.value(SCISSORS, PAPER), 1.0);
    assert_eq!(rps_enviroment.value(SCISSORS, SCISSORS), 0.0);
    assert_eq!(rps_enviroment.value(SCISSORS, ROCK), -1.0);
}
