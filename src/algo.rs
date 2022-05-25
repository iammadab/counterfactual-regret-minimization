use rand::Rng;

pub struct CFR {
    /// Keeps track of how much regret the player has had for not taking an action
    action_regret_over_time: Vec<f64>,
    strategy: Vec<f64>,
    strategy_sum: Vec<f64>,
    no_of_actions: usize,
}

impl CFR {
    pub fn new(no_of_actions: usize) -> Self {
        Self {
            action_regret_over_time: vec![0.0; no_of_actions],
            strategy: vec![0.0; no_of_actions],
            strategy_sum: vec![0.0; no_of_actions],
            no_of_actions: no_of_actions,
        }
    }

    fn get_strategy(&mut self) -> &Vec<f64> {
        let mut normalizing_sum = 0.0;

        // Set the strategy to be equal to the regret of each action over time
        // If the regret is negative make it zero
        // Then keep track of the sum of all the strateg-ies
        // So that we can normalize the strategy later on (i.e turn it into a probabilty distribution)
        for i in 0..self.no_of_actions {
            if self.action_regret_over_time[i] > 0.0 {
                self.strategy[i] = self.action_regret_over_time[i];
            } else {
                self.strategy[i] = 0.0;
            }
            normalizing_sum += self.strategy[i];
        }

        for i in 0..self.no_of_actions {
            // If there is a normalizer, normalize all the strategy values
            // Else give all the strategies equal probabilities
            // Regardless, sum all the strategies into a strategy sum (Why??)
            if normalizing_sum > 0.0 {
                self.strategy[i] /= normalizing_sum;
            } else {
                self.strategy[i] = 1.0 / self.no_of_actions as f64;
            }
            self.strategy_sum[i] += self.strategy[i];
        }

        &self.strategy
    }

    /// The average strategy over time is what converges to the nash equilibrium
    // TODO: Figure out why this is the case
    pub fn get_average_strategy(&mut self) -> Vec<f64> {
        let mut normalizing_sum = 0.0;
        let mut average_strategy = vec![0.0; self.no_of_actions];

        for i in 0..self.no_of_actions {
            normalizing_sum += self.strategy_sum[i];
        }

        for i in 0..self.no_of_actions {
            if normalizing_sum > 0.0 {
                average_strategy[i] = self.strategy_sum[i] / normalizing_sum;
            } else {
                average_strategy[i] = 1.0 / self.no_of_actions as f64;
            }
        }
        average_strategy
    }

    /// Choose an action based on some prefered action probability distribution defined
    /// by a given strategy.
    fn get_action(no_of_actions: usize, strategy: &Vec<f64>) -> usize {
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

    pub fn get_move(&mut self) -> usize {
        CFR::get_action(self.no_of_actions, &self.get_strategy())
    }

    pub fn train(&mut self, my_action: usize, action_utilities: Vec<f64>) {
        // Take feedback from the environment (action_utilities)
        // use this feedback to update the regret values
        for i in 0..self.no_of_actions {
            self.action_regret_over_time[i] += action_utilities[i] - action_utilities[my_action];
        }
    }
}
