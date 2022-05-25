use std::collections::HashMap;
use crate::Game;

pub struct ColonelBlotto {
    pub actions: HashMap<usize, Vec<usize>>,
}

impl ColonelBlotto {
    pub fn new() -> Self {
        let mut actions: HashMap<usize, Vec<usize>> = HashMap::new();
        let troop_size: usize = 5;
        let mut curr_index: usize = 0;

        for i in 0..=troop_size {
            for j in 0..=troop_size {
                for k in 0..=troop_size {
                    if i + j + k < troop_size {
                        actions.insert(curr_index, vec![i, j, k]);
                        curr_index += 1;
                    }
                }
            }
        }

        ColonelBlotto { actions }
    }
}

impl Game for ColonelBlotto {
    type Action = Vec<usize>;

    fn no_of_actions(&self) -> usize {
        self.actions.len()
    }

    fn value(&self, action1: Self::Action, action2: Self::Action) -> f64 {
        let (mut score1, mut score2) = (0, 0);
        for i in 0..action1.len() {
            if action1[i] > action2[i] {
                score1 += 1;
            } else if action1[i] < action2[i] {
                score2 += 1;
            }
        }
        if score1 > score2 {
            return 1.0;
        } else if score1 < score2 {
            return -1.0;
        }
        return 0.0;
    }

    fn action_utilities(&self, opponent_action: usize) -> Vec<f64> {
        let mut action_utilities = vec![0.0; self.no_of_actions()];
        for i in 0..self.no_of_actions() {
            action_utilities[i] =
                self.value(self.actions[&i].clone(), self.actions[&opponent_action].clone());
        }
        action_utilities
    }
}



#[test]
fn test_value_function() {
    let colonel_blotto_environment = ColonelBlotto::new();
    assert_eq!(colonel_blotto_environment.value(vec![5, 2, 3], vec![6, 5, 2]), -1.0);
    assert_eq!(colonel_blotto_environment.value(vec![5, 2, 3], vec![5, 2, 3]), 0.0);
    assert_eq!(colonel_blotto_environment.value(vec![1, 5, 2], vec![1, 5, 1]), 1.0);
}
