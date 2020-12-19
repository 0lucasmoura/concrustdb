use petgraph::Graph;
//use petgraph::graph::Node;
use petgraph::graph::NodeIndex;
use console_engine::ConsoleEngine;
use console_engine::pixel;
use console_engine::Color;
use std::collections::HashMap;

pub struct GraphHandler<'a>{
    graph: Graph::<&'a str, i8>,
    drawed_nodes: HashMap<usize, (i32, i32)>,
    screen_height: i32,
    screen_width: i32
}

impl <'a> GraphHandler<'a> {

    pub fn init (screen_height: i32, screen_width: i32) -> GraphHandler<'a> {
        GraphHandler{
            graph: Graph::<&str, i8>::new(),
            drawed_nodes: HashMap::new(),
            screen_height: screen_height,
            screen_width: screen_width
        }
    }

    fn _hello() -> (){}

    pub fn draw_neighbors (&mut self, engine: &mut ConsoleEngine) -> (){
        let placement_x:i32 = 10;
        let placement_y:i32 = 10;

        for node_ix in self.graph.node_indices(){
            let node_iter = self.graph.neighbors(node_ix);

            let mut positive:bool = false;

            if !self.drawed_nodes.contains_key(&node_ix.index()){
                self.drawed_nodes.insert(node_ix.index(), (placement_x, placement_y));
                engine.set_pxl(
                    placement_x,
                    placement_y,
                    pixel::pxl_fg('O', Color::Red),
                );
            }
            engine.set_pxl(
                placement_x,
                placement_y,
                pixel::pxl_fg('O', Color::Red),
            );

            //println!("--------- ix: {} ---------", node_ix.index());
            //println!("{}", self.graph[node_ix]);
            for node in node_iter{

                //if node_iter.size_hint() > (0, Some(1)) {

                //}
                //if !self.drawed_nodes.contains_key(&node.index()){
                if positive {
                    self.drawed_nodes.insert(node.index(), (self.drawed_nodes[&node_ix.index()].0 + 5, 
                                                            self.drawed_nodes[&node_ix.index()].1 + 3));
                    engine.set_pxl(
                        self.drawed_nodes[&node_ix.index()].0 + 5,
                        self.drawed_nodes[&node_ix.index()].1 + 3,
                        pixel::pxl_fg('O', Color::Red));
                    positive = false;
                } else{
                    self.drawed_nodes.insert(node.index(), (self.drawed_nodes[&node_ix.index()].0 + 5, 
                                                            self.drawed_nodes[&node_ix.index()].1 - 3));
                    engine.set_pxl(
                        self.drawed_nodes[&node_ix.index()].0 + 5,
                        self.drawed_nodes[&node_ix.index()].1 - 3,
                        pixel::pxl_fg('O', Color::Red));
                    positive = true;
                }
                //}

                engine.line(
                    self.drawed_nodes[&node_ix.index()].0 + 1 as i32,
                    self.drawed_nodes[&node_ix.index()].1  as i32,
                    self.drawed_nodes[&node.index()].0 - 1 as i32,
                    self.drawed_nodes[&node.index()].1 as i32,
                    pixel::pxl_fg(
                        '>',
                        Color::Rgb {
                            r: rand::random::<u8>(),
                            g: rand::random::<u8>(),
                            b: rand::random::<u8>(),
                        },
                    ),
                );
                //count_y += 5;
                //println!("Vizinhos -> {}", node.index());
            }
            //count_x += 5;
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
