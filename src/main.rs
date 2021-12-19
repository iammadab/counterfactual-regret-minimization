use crate::colonel_blotto::ColonelBlotto;

mod algo;
mod rps;
mod colonel_blotto;

fn main() {
    // train_rps();
    train_colonel_blotto();
}

fn train_rps(){
    let mut player1 = algo::CFR::new(rps::RPS::NO_OF_ACTIONS);
    let mut player2 = algo::CFR::new(rps::RPS::NO_OF_ACTIONS);

    const ITERATIONS: usize = 1000000;

    println!("Training..");

    for i in 0..ITERATIONS {
        let player1_move = player1.getMove();
        let player2_move = player2.getMove();

        player1.train(player1_move, rps::RPS::action_utilities(player2_move));
        player2.train(player2_move, rps::RPS::action_utilities(player1_move));
    }

    println!();
    println!("Player 1 strategy: {:?}", player1.getAverageStrategy());
    println!("Player 2 strategy: {:?}", player2.getAverageStrategy());
}

fn train_colonel_blotto() {
    let colonel_blotto_instance = ColonelBlotto::new();
    let mut player1 = algo::CFR::new(colonel_blotto_instance.no_of_actions());
    let mut player2 = algo::CFR::new(colonel_blotto_instance.no_of_actions());

    const ITERATIONS: usize = 1000000;

    println!("Training");

    for i in 0..ITERATIONS {
        let player1_move = player1.getMove();
        let player2_move = player2.getMove();

        player1.train(player1_move, colonel_blotto_instance.action_utilities(player2_move));
        player2.train(player2_move, colonel_blotto_instance.action_utilities(player1_move));
    }

    println!();
    println!("Player 1 strategy: {:?}", player1.getAverageStrategy());
    println!("Player 2 strategy: {:?}", player2.getAverageStrategy());
}