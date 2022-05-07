pub mod system;

fn main() {
    println!("Atlas' Uplink System Core");
    let mut state = system::State::new();
    system::init(&mut state);
    system::start_game_loop(&mut state);
}
