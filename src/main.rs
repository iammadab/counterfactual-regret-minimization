use crate::colonel_blotto::ColonelBlotto;
use crate::rps::{Game, RPS};
use lars::vector;
use lars::vector::Vector;

mod algo;
mod colonel_blotto;
mod rps;

fn main() {
    let rps_game = RPS {};
    let mut player1 = algo::CFR::new(rps_game.no_of_actions());
    let mut player2 = algo::CFR::new(rps_game.no_of_actions());
    train_till_convergence(&mut player1, &mut player2, rps_game);
    // train(&mut player1, &mut player2, rps_game);

    // let colonel_blotto_game = ColonelBlotto::new();
    // let mut player1 = algo::CFR::new(colonel_blotto_game.no_of_actions());
    // let mut player2 = algo::CFR::new(colonel_blotto_game.no_of_actions());
    // train_till_convergence(&mut player1, &mut player2, colonel_blotto_game);

    // train(&mut player1, &mut player2, colonel_blotto_game);
}

const ITERATIONS: usize = 100000;

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

fn train_till_convergence(
    player_1: &mut algo::CFR,
    player_2: &mut algo::CFR,
    environment: impl Game,
) {
    println!("Traning..");

    let mut last_strategy: Vector<f64> = vector::from(&player_1.get_average_strategy());

    for _ in 0..ITERATIONS {
        let player_1_move = player_1.get_move();
        let player_2_move = player_2.get_move();

        player_1.train(player_1_move, environment.action_utilities(player_2_move));
        player_2.train(player_2_move, environment.action_utilities(player_1_move));

        let new_strategy = vector::from(&player_1.get_average_strategy());
        let diff = new_strategy.clone() - last_strategy.clone();
        println!("{}", diff);
        last_strategy = new_strategy;
    }

    // let sa = vector::from(&player_1.get_average_strategy());
    // let sb = vector::from(&player_2.get_average_strategy());
    // let sc = sa - sb;
    // sc.content.iter().map()
    // println!("{}", sc);
}
