use petgraph::Graph;
//use petgraph::graph::Node;
use petgraph::graph::NodeIndex;
use console_engine::ConsoleEngine;
use console_engine::pixel;
use console_engine::Color;


pub struct GraphHandler<'a>{
    graph: Graph::<&'a str, i8>,
    drawed_nodes: Vec<usize>
}

impl <'a> GraphHandler<'a> {

    pub fn init () -> GraphHandler<'a> {
        GraphHandler{
            graph: Graph::<&str, i8>::new(),
            drawed_nodes: [].to_vec()
        }
    }

    fn hello() -> (){}


    pub fn draw_neighbors (&mut self, engine: &mut ConsoleEngine) -> (){
        let mut count_x = 0;
        let mut count_y = 0;

        for node_ix in self.graph.node_indices(){
            let node_iter = self.graph.neighbors(node_ix);

            engine.set_pxl(
                10 + count_x,
                10 + count_y,
                pixel::pxl_fg('O', Color::Red),
            );

            count_y = 0;

            if !self.drawed_nodes.contains(&node_ix.index()){
                self.drawed_nodes.push(node_ix.index())
            }

            //println!("--------- ix: {} ---------", node_ix.index());
            //println!("{}", self.graph[node_ix]);
            for node in node_iter{
                count_y += 5;
                //println!("Vizinhos -> {}", node.index());
            }
            count_x += 5;
        }
    }

    pub fn add_neighbor (&mut self, node_start: NodeIndex, new_node_name: &'a str) -> NodeIndex{
        let new_node = self.graph.add_node(new_node_name);
        self.graph.add_edge(node_start, new_node, 0);

        return new_node;
    }

    pub fn add_first_node(&mut self, node_name: &'a str) -> NodeIndex {
        return self.graph.add_node(node_name);
    }
}
