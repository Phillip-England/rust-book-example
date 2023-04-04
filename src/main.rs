mod examples;
use examples::data_types::data_types;
use examples::guessing_game::guessing_game;
use examples::hello_world::hello_world;
use examples::variables::variables;
use examples::functions::functions;
use examples::control_flow::control_flow;
use examples::ownership::ownership;
use examples::borrowing::borrowing;
use examples::slices::slices;
use examples::structs::structs;

fn main() {
    let test: bool = false;
    if test {
        hello_world();
        guessing_game();
        variables();
        data_types();
        functions();
        control_flow();
        ownership();
        borrowing();
        slices();
    }
    structs();
}
