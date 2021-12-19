use rand::Rng;

const NUM_OF_ACTIONS: usize = 3;
const ROCK: usize = 0;
const PAPER: usize = 1;
const SCISSORS: usize = 2;

fn main() {
    let mut player1 = RPS_CFR::new();
    let mut player2 = RPS_CFR::new();

    const ITERATIONS: usize = 1000000;

    println!("Training..");

    for i in 0..ITERATIONS {
        let player1_move = player1.getMove();
        let player2_move = player2.getMove();

        player1.train(player1_move, player2_move);
        player2.train(player2_move, player1_move);
    }

    println!();
    println!("Player 1 strategy: {:?}", player1.getAverageStrategy());
    println!("Player 2 strategy: {:?}", player2.getAverageStrategy());
}

struct RPS_CFR {
    regretSum: Vec<f64>,
    strategy: Vec<f64>,
    strategySum: Vec<f64>,
}

impl RPS_CFR {
    pub fn new() -> Self {
        Self {
            regretSum: vec![0.0, 0.0, 0.0],
            strategy: vec![0.0, 0.0, 0.0],
            strategySum: vec![0.0, 0.0, 0.0],
        }
    }

    fn getStrategy(&mut self) -> Vec<f64> {
        let mut normalizingSum = 0.0;
        // Set the strategy to be equal to the regret sum
        // If the regret is negative make it zero
        // Then keep track of the sum of all the strategies
        // So that we can normalize the strategy later on
        for i in 0..NUM_OF_ACTIONS {
            if self.regretSum[i] > 0.0 {
                self.strategy[i] = self.regretSum[i];
            } else {
                self.strategy[i] = 0.0;
            }
            normalizingSum += self.strategy[i];
        }
        // If there is a normalizer, normalize all the strategy values
        // Else give all the strategies equal probabilities
        // Regardless, sum all the strategies into a strategy sum (Why??)
        for i in 0..NUM_OF_ACTIONS {
            if normalizingSum > 0.0 {
                self.strategy[i] /= normalizingSum;
            } else {
                self.strategy[i] = 1.0 / NUM_OF_ACTIONS as f64;
            }
            self.strategySum[i] += self.strategy[i];
        }
        self.strategy.clone()
    }

    pub fn getAverageStrategy(&mut self) -> Vec<f64> {
        let mut normalizingSum = 0.0;
        let mut averageStrategy = vec![0.0, 0.0, 0.0];

        for i in 0..NUM_OF_ACTIONS {
            normalizingSum += self.strategySum[i];
        }

        for i in 0..NUM_OF_ACTIONS {
            if normalizingSum > 0.0 {
                averageStrategy[i] = self.strategySum[i] / normalizingSum;
            } else {
                averageStrategy[i] = 1.0 / NUM_OF_ACTIONS as f64;
            }
        }
        averageStrategy
    }

    fn getAction(strategy: &Vec<f64>) -> usize {
        let r: f64 = rand::thread_rng().gen();
        let mut action: usize = 0;
        let mut cummulativeProbability = 0.0;
        while action < NUM_OF_ACTIONS - 1 {
            cummulativeProbability += strategy[action];
            if r < cummulativeProbability {
                break;
            }
            action += 1;
        }
        action
    }

    pub fn getMove(&mut self) -> usize {
        RPS_CFR::getAction(&self.getStrategy())
    }

    pub fn train(&mut self, my_action: usize, opponent_action: usize) {
        let mut action_utilities: Vec<f64> = vec![0.0, 0.0, 0.0];
        for i in 0..NUM_OF_ACTIONS {
            action_utilities[i] = RPS_CFR::value(i, opponent_action);
        }
        for i in 0..NUM_OF_ACTIONS {
            self.regretSum[i] += action_utilities[i] - action_utilities[my_action];
        }
    }

    pub fn value(action1: usize, action2: usize) -> f64 {
        if action1 == action2 {
            return 0.0;
        }
        if ((action1 + 1) % 3) == action2 {
            return -1.0;
        }
        return 1.0;
    }
}

#[test]
fn test_value_function() {
    // Exhaust Rock
    assert_eq!(RPS_CFR::value(ROCK, PAPER), -1.0);
    assert_eq!(RPS_CFR::value(ROCK, SCISSORS), 1.0);
    assert_eq!(RPS_CFR::value(ROCK, ROCK), 0.0);

    // Exhaust Paper
    assert_eq!(RPS_CFR::value(PAPER, PAPER), 0.0);
    assert_eq!(RPS_CFR::value(PAPER, SCISSORS), -1.0);
    assert_eq!(RPS_CFR::value(PAPER, ROCK), 1.0);

    // Exhaust Scissors
    assert_eq!(RPS_CFR::value(SCISSORS, PAPER), 1.0);
    assert_eq!(RPS_CFR::value(SCISSORS, SCISSORS), 0.0);
    assert_eq!(RPS_CFR::value(SCISSORS, ROCK), -1.0);
}
