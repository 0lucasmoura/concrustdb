extern crate console_engine;
extern crate petgraph;
mod graph_handler;
use petgraph::Graph;
//use petgraph::graph::Node;

//use console_engine::ConsoleEngine;
use console_engine::KeyCode;
//use petgraph::visit::WalkerIter;
//use petgraph::No
//use petgraph::visit::Dfs;

fn main() {
    let mut engine = console_engine::ConsoleEngine::init_fill_require(10, 10, 4);

    let mut graph_h = graph_handler::GraphHandler::init();

    let first_node = graph_h.add_first_node("start");
    let teste = graph_h.add_neighbor(first_node, "teste");
    let teste1 = graph_h.add_neighbor(first_node, "teste1");
    let teste2 = graph_h.add_neighbor(teste1, "teste2");
    let teste3 = graph_h.add_neighbor(teste2, "teste3");


    loop {

        engine.wait_frame(); // wait for next frame + capture inputs
        engine.check_resize();

        // exit check
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }

        engine.clear_screen(); // reset the screen
        graph_h.draw_neighbors(&mut engine);

        let height = engine.get_height()/2;
        let width = engine.get_width()/2;

        engine.print(width as i32, height as i32, "Hello, World!"); // , Color::Yellow, Color::Black);

        engine.draw(); // draw the screen
    }
}
