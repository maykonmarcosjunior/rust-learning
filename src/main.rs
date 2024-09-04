mod guessing_game;
mod variables;
mod send_receive;
mod bee_1853_game_of_trust;

fn main() {
    variables::run();
    println!("Hello, World");
    send_receive::run();
    guessing_game::run();
}
