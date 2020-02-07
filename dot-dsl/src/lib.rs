pub mod graph {
    use std::collections::HashMap;
    use crate::graph::graph_items::node::Node;

    pub struct Graph {
        pub  nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: std::collections::HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[graph_items::node::Node]) -> Graph {
            let n: Vec<graph_items::node::Node> = nodes.to_vec();
            self.nodes = n;
            self
        }
        pub fn with_edges(mut self, edges: &[graph_items::edge::Edge]) -> Graph {
            let e: Vec<graph_items::edge::Edge> = edges.to_vec();
            self.edges = e;
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Graph {
            for attr in attrs {
                self.attrs.insert(attr.0.to_string(), attr.1.to_string());
            }
            self
        }

        pub fn get_node(&self, name: &str) -> Option<&Node> {
            for node in &self.nodes {
                if node.name.eq(name.into()) {
                    return Some(node);
                }
            }
            None
        }
    }


    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Eq, PartialEq, Debug)]
            pub struct Edge {
                n1: &'static str,
                n2: &'static str,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(n1: &'static str, n2: &'static str) -> Edge {
                    Edge {
                        n1,
                        n2,
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Edge {
                    for attr in attrs {
                        self.attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }
                    self
                }
            }
        }

        use std::collections::HashMap;
        pub mod node {
            use std::collections::HashMap;
            #[derive(Clone, Eq, PartialEq, Debug)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Node {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&'static str, &'static str)]) -> Node {
                    for attr in attrs {
                        self.attrs.insert(attr.0.into(), attr.1.into());
                    }
                    self
                }

                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    let attr = self.attrs.get(name)?;
                    Some(attr)
                }
            }
        }
    }
}
