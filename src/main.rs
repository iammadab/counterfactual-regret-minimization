use crate::colonel_blotto::ColonelBlotto;
use crate::rps::{Game, RPS};

mod algo;
mod colonel_blotto;
mod rps;

fn main() {
    // let rps_game = RPS{};
    // let mut player1 = algo::CFR::new(rps_game.no_of_actions());
    // let mut player2 = algo::CFR::new(rps_game.no_of_actions());
    // train(&mut player1, &mut player2, rps_game);

    let colonel_blotto_game = ColonelBlotto::new();
    let mut player1 = algo::CFR::new(colonel_blotto_game.no_of_actions());
    let mut player2 = algo::CFR::new(colonel_blotto_game.no_of_actions());
    train(&mut player1, &mut player2, colonel_blotto_game);
}

const ITERATIONS: usize = 100000;

// pub struct Trainer {
//     environment: Box<dyn Game>,
// }
//
// impl Trainer {
//     fn train(&self){
//
//     }
// }

fn train_colonel_blotto() {
    let colonel_blotto_instance = ColonelBlotto::new();
    let mut player1 = algo::CFR::new(colonel_blotto_instance.no_of_actions());
    let mut player2 = algo::CFR::new(colonel_blotto_instance.no_of_actions());

    println!("Training");

    for _ in 0..ITERATIONS {
        let player1_move = player1.get_move();
        let player2_move = player2.get_move();

        player1.train(
            player1_move,
            colonel_blotto_instance.action_utilities(player2_move),
        );
        player2.train(
            player2_move,
            colonel_blotto_instance.action_utilities(player1_move),
        );
    }

    println!();
    println!("Player 1 strategy: {:?}", player1.get_average_strategy());
    println!("Player 2 strategy: {:?}", player2.get_average_strategy());
}

fn train(player_1: &mut algo::CFR, player_2: &mut algo::CFR, environment: impl Game) {
    println!("Traning..");

    for _ in 0..ITERATIONS {
        let player_1_move = player_1.get_move();
        let player_2_move = player_2.get_move();

        player_1.train(player_1_move, environment.action_utilities(player_2_move));
        player_2.train(player_2_move, environment.action_utilities(player_1_move));
        println!("Player 1 strategy {:?}", player_1.get_average_strategy());
    }
    println!();
    println!("Player 1 strategy {:?}", player_1.get_average_strategy());
    println!("Player 2 strategy {:?}", player_2.get_average_strategy());
}
