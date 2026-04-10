#[derive(Clone, Debug, PartialEq)]
pub enum DataContainer {
    Array(Vec<i32>),
    Tree {
        nodes: Vec<i32>, // valores de los nodos, no ocupamos apuntadores a ellos como en c
        edges: Vec<(usize, usize)>, // direcciones implicitas (Padre, Hijo)
        root: usize,
    },
    Graph {
        nodes: Vec<i32>,
        edges: Vec<Edge>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub weigh: Option<i32>,
    pub directed: bool,
}