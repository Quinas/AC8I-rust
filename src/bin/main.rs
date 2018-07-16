extern crate chip8_int;

fn main() {
    let mut system = chip8_int::system::System::new();
    system.load_game("airplane.ch8");
    system.emulate();
}
