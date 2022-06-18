// https://aloso.github.io/2021/03/28/module-system.html
// https://www.sheshbabu.com/posts/rust-module-system/
pub mod graph {
    use graph_items::{edge::Edge, node::Node};
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Default::default()
        }
        // https://rust-unofficial.github.io/patterns/patterns/creational/builder.html
        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }
        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            self
        }
        pub fn get_node(self, val: &str) -> Option<Node> {
            self.nodes.iter().find(|n| n.0 == val).cloned()
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge(String, String, HashMap<String, String>);

            impl Edge {
                pub fn new(tail: &str, head: &str) -> Self {
                    Self(tail.to_string(), head.to_string(), HashMap::new())
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.2 = attrs
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();
                    self
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node(pub String, HashMap<String, String>);

            impl Node {
                pub fn new(val: &str) -> Self {
                    Self(val.to_string(), HashMap::new())
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.1 = attrs
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();
                    self
                }
                // If we take ownership of the instance, we have to work lot harder
                // pub fn get_attr(self, name: &str) -> Option<&str> {
                //     self.1
                //         .get(name)
                //         // Create a &str that'll live until the program is terminated
                //         .map(|v| Box::leak(v.clone().into_boxed_str()))
                //         // Convert &mut str to &str
                //         .map(|v| {
                //             let s = v;
                //             &*s
                //         })
                // }
                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    self.1.get(name).map(|x| x.as_str())
                }
            }
        }
    }
}
