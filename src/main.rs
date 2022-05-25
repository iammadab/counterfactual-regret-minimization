use crate::colonel_blotto::ColonelBlotto;

mod algo;
mod colonel_blotto;
mod rps;

fn main() {
    train_rps();
    // train_colonel_blotto();
}

const ITERATIONS: usize = 100000;

fn train_rps() {
    let mut player1 = algo::CFR::new(rps::RPS::NO_OF_ACTIONS);
    let mut player2 = algo::CFR::new(rps::RPS::NO_OF_ACTIONS);

    println!("Training..");

    for _ in 0..ITERATIONS {
        let player1_move = player1.get_move();
        let player2_move = player2.get_move();

        player1.train(player1_move, rps::RPS::action_utilities(player2_move));
        player2.train(player2_move, rps::RPS::action_utilities(player1_move));
    }

    println!();
    println!("Player 1 strategy: {:?}", player1.get_average_strategy());
    println!("Player 2 strategy: {:?}", player2.get_average_strategy());
}

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
