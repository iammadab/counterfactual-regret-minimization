const NUM_OF_ACTIONS: usize = 3;

fn main(){
    let mut instance = RPS_CFR::new();
    println!("{:?}", instance.getStrategy());
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

    pub fn getStrategy(&mut self) -> Vec<f64> {
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
}