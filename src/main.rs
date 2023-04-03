mod exmaples;
use exmaples::data_types::data_types;
use exmaples::guessing_game::guessing_game;
use exmaples::hello_world::hello_world;
use exmaples::variables::variables;
use exmaples::functions::functions;
use exmaples::control_flow::control_flow;
use exmaples::ownership::ownership;

fn main() {
    let test: bool = false;

    if test {
        hello_world();
        guessing_game();
        variables();
        data_types();
        functions();
        control_flow();
    }

    ownership();


}
