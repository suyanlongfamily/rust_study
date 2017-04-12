// use generic parameters
// trait Graph<N, E> {
//     fn has_edge(&self, &N, &N) -> bool;
//     fn edges(&self, &N) -> Vec<E>;
// }
//这个是参数泛型，编译期间进行扩充，
// fn distance<N, E, G: Graph<N, E>>(graph: &G, start: &N, end: &N) -> u32 {}

// use associated types
trait Graph {
    type N;
    type E; //关联类型

    fn has_edge(&self, &Self::N, &Self::N) -> bool;
    fn edges(&self, &Self::N) -> Vec<Self::E>;
}

//这个是特性对象引用，参数多态。
fn distance<G>(graph: &G) -> u32
    where G: Graph{

    0
}

struct Node;

struct Edge;

struct SimpleGraph;


impl Graph for SimpleGraph {
    type N = Node;
    type E = Edge;

    fn has_edge(&self, n1: &Node, n2: &Node) -> bool {
        false
    }
    fn edges(&self, n: &Node) -> Vec<Edge> {
        let vec: Vec<Edge> = vec![];
        vec
    }
}

fn main() {
    let graph = SimpleGraph;
    distance(&graph);
    
    // let object = Box::new(graph) as Box<Graph<N = Node, E = Edge>>;
    // distance(object);


}
