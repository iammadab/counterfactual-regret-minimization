use rand::Rng;

pub struct CFR {
    regretSum: Vec<f64>,
    strategy: Vec<f64>,
    strategySum: Vec<f64>,
    noOfActions: usize,
}

impl CFR {
    pub fn new(no_of_actions: usize) -> Self {
        Self {
            regretSum: vec![0.0; no_of_actions],
            strategy: vec![0.0; no_of_actions],
            strategySum: vec![0.0; no_of_actions],
            noOfActions: no_of_actions,
        }
    }

    fn getStrategy(&mut self) -> Vec<f64> {
        let mut normalizingSum = 0.0;
        // Set the strategy to be equal to the regret sum
        // If the regret is negative make it zero
        // Then keep track of the sum of all the strategies
        // So that we can normalize the strategy later on
        for i in 0..self.noOfActions {
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
        for i in 0..self.noOfActions {
            if normalizingSum > 0.0 {
                self.strategy[i] /= normalizingSum;
            } else {
                self.strategy[i] = 1.0 / self.noOfActions as f64;
            }
            self.strategySum[i] += self.strategy[i];
        }
        self.strategy.clone()
    }

    pub fn getAverageStrategy(&mut self) -> Vec<f64> {
        let mut normalizingSum = 0.0;
        let mut averageStrategy = vec![0.0, 0.0, 0.0];

        for i in 0..self.noOfActions {
            normalizingSum += self.strategySum[i];
        }

        for i in 0..self.noOfActions {
            if normalizingSum > 0.0 {
                averageStrategy[i] = self.strategySum[i] / normalizingSum;
            } else {
                averageStrategy[i] = 1.0 / self.noOfActions as f64;
            }
        }
        averageStrategy
    }

    fn getAction(no_of_actions: usize, strategy: &Vec<f64>) -> usize {
        let r: f64 = rand::thread_rng().gen();
        let mut action: usize = 0;
        let mut cummulativeProbability = 0.0;
        while action < no_of_actions - 1 {
            cummulativeProbability += strategy[action];
            if r < cummulativeProbability {
                break;
            }
            action += 1;
        }
        action
    }

    pub fn getMove(&mut self) -> usize {
        CFR::getAction(self.noOfActions, &self.getStrategy())
    }

    pub fn train(&mut self, my_action: usize, action_utilities: Vec<f64>) {
        for i in 0..self.noOfActions {
            self.regretSum[i] += action_utilities[i] - action_utilities[my_action];
        }
    }
}
