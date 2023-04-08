mod examples;
use std::process;

use examples::borrowing::borrowing;
use examples::collections::collections;
use examples::control_flow::control_flow;
use examples::data_types::data_types;
use examples::enums::enums;
use examples::errors::errors;
use examples::functions::functions;
use examples::generics_traits_lifetimes::generic_traits_lifetimes;
use examples::guessing_game::guessing_game;
use examples::hello_world::hello_world;
use examples::ownership::ownership;
use examples::slices::slices;
use examples::structs::structs;
use examples::variables::variables;
use examples::mini_grep::mini_grep;

pub fn run() {
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
        structs();
        enums();
        collections();
        errors();
        generic_traits_lifetimes();
    }
    if let Err(e) = mini_grep() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
