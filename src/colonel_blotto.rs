use std::collections::HashMap;

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

    pub fn no_of_actions(&self) -> usize {
        self.actions.len()
    }

    fn value(action1: &Vec<usize>, action2: &Vec<usize>) -> f64 {
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

    pub fn action_utilities(&self, opponent_action: usize) -> Vec<f64> {
        let mut action_utilities = vec![0.0; self.no_of_actions()];
        for i in 0..self.no_of_actions() {
            action_utilities[i] =
                ColonelBlotto::value(&self.actions[&i], &self.actions[&opponent_action]);
        }
        action_utilities
    }
}

#[test]
fn test_value_function() {
    assert_eq!(ColonelBlotto::value(&vec![5, 2, 3], &vec![6, 5, 2]), -1.0);
    assert_eq!(ColonelBlotto::value(&vec![5, 2, 3], &vec![5, 2, 3]), 0.0);
    assert_eq!(ColonelBlotto::value(&vec![1, 5, 2], &vec![1, 5, 1]), 1.0);
}
